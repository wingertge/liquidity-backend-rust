use std::{fmt, thread};
use tic::{Sample, Receiver, Sender, Clocksource, Interest, HttpReporter};
use crate::graphql::context::Context;
use crate::auth::JWTError;
use juniper::FieldResult;
use std::sync::Arc;
use std::net::SocketAddr;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Metric {
    JWT,
    Total,
    Query,
    Mutation,
    Ok,
    Error
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Metric::Total => write!(f, "total"),
            Metric::Query => write!(f, "query"),
            Metric::Mutation => write!(f, "mutation"),
            Metric::JWT => write!(f, "jwt parsing"),
            Metric::Error => write!(f, "error"),
            Metric::Ok => write!(f, "ok")
        }
    }
}

pub trait GQLMonitor {
    fn monitor<'a, T, F>(&'a self, metric: Metric, f: F) -> FieldResult<T>
        where F: (FnOnce(&'a Context) -> FieldResult<T>);
/*    fn monitor_async<'a, T, F, Fut>(&self, metric: Metric, f: F)
        -> Pin<Box<dyn Future<Output=T> + Send + 'a>>
        where
            F: Fn(&'a Context) -> Fut,
            Fut: Future<Output=T>,
            Self: Sized;*/
}

impl GQLMonitor for Result<Context, JWTError> {
    fn monitor<'a, T, F>(&'a self, metric: Metric, f: F) -> FieldResult<T>
        where F: (FnOnce(&'a Context) -> FieldResult<T>) {
        match &self {
            Ok(context) => {
                let clocksource = context.metrics.clocksource();
                let mut sender = context.metrics.sender();

                let start = clocksource.counter();
                let res = f(&context);
                let stop = clocksource.counter();
                log::info!("Start: {}, Stop: {}", start, stop);
                sender.send(Sample::new(start, stop, metric)).unwrap();
                sender.send(Sample::new(start, stop, Metric::Total)).unwrap();
                match res.is_err() {
                    true => sender.send(Sample::new(start, stop, Metric::Error)).unwrap(),
                    false => sender.send(Sample::new(start, stop, Metric::Ok)).unwrap()
                };
                res
            },
            Err(e) => Err(e.into())
        }
    }

/*    fn monitor_async<'a, T, F, Fut>(&self, metric: Metric, f: F)
        -> Pin<Box<dyn Future<Output=T> + Send + 'a>>
        where
            F: FnOnce(&'a Context) -> Fut,
            Fut: Future<Output=T>,
            Self: Sized, T: Send {
        match self {
            &Err(e) => Box::pin(futures::future::ready(Err(e.into()))),
            &Ok(context) => {
                let clocksource = context.metrics_clocksource.clone();
                let mut sender = context.metrics_sender.clone();

                let start = clocksource.counter();
                let res = f(&context).then(|res| {
                    let stop = clocksource.counter();
                    sender.send(Sample::new(start, stop, metric)).unwrap();
                    sender.send(Sample::new(start, stop, Metric::Total)).unwrap();
                    res
                });
                Box::pin(res)
            }
        }
    }*/
}

pub struct Metrics {
    sender: Sender<Metric>,
    clocksource: Clocksource
}

impl Metrics {
    pub fn new(reporting_addr: SocketAddr) -> (Receiver<Metric>, Arc<Metrics>) {
        let mut receiver = Receiver::configure()
            .batch_size(4)
            .service(true)
            .build();

        let mut reporter = HttpReporter::new(&receiver, reporting_addr);
        thread::spawn(move || reporter.run());

        receiver.add_interest(Interest::Count(Metric::Ok));
        receiver.add_interest(Interest::LatencyPercentile(Metric::Ok));
        receiver.add_interest(Interest::Count(Metric::Error));
        receiver.add_interest(Interest::Count(Metric::Total));
        receiver.add_interest(Interest::Count(Metric::Query));
        receiver.add_interest(Interest::LatencyPercentile(Metric::Query));
        receiver.add_interest(Interest::Count(Metric::Mutation));
        receiver.add_interest(Interest::LatencyPercentile(Metric::Mutation));
        receiver.add_interest(Interest::LatencyPercentile(Metric::JWT));

        let metrics = Metrics {
            sender: receiver.get_sender(),
            clocksource: receiver.get_clocksource()
        };

        (receiver, Arc::new(metrics))
    }

    pub fn sender(&self) -> Sender<Metric> { self.sender.clone() }
    pub fn clocksource(&self) -> Clocksource { self.clocksource.clone() }
}