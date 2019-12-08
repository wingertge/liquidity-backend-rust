(function() {var implementors = {};
implementors["hyper"] = [{text:"impl <a class=\"trait\" href=\"hyper/rt/trait.Stream.html\" title=\"trait hyper::rt::Stream\">Stream</a> for <a class=\"struct\" href=\"hyper/struct.Body.html\" title=\"struct hyper::Body\">Body</a>",synthetic:false,types:["hyper::body::body::Body"]},{text:"impl&lt;I, S, B, E&gt; <a class=\"trait\" href=\"hyper/rt/trait.Stream.html\" title=\"trait hyper::rt::Stream\">Stream</a> for <a class=\"struct\" href=\"hyper/server/conn/struct.Serve.html\" title=\"struct hyper::server::conn::Serve\">Serve</a>&lt;I, S, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"hyper/rt/trait.Stream.html\" title=\"trait hyper::rt::Stream\">Stream</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::<a class=\"type\" href=\"hyper/rt/trait.Stream.html#associatedtype.Item\" title=\"type hyper::rt::Stream::Item\">Item</a>: <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::<a class=\"type\" href=\"hyper/rt/trait.Stream.html#associatedtype.Error\" title=\"type hyper::rt::Stream::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: MakeServiceRef&lt;I::<a class=\"type\" href=\"hyper/rt/trait.Stream.html#associatedtype.Item\" title=\"type hyper::rt::Stream::Item\">Item</a>, ReqBody = <a class=\"struct\" href=\"hyper/struct.Body.html\" title=\"struct hyper::Body\">Body</a>, ResBody = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"hyper/body/trait.Payload.html\" title=\"trait hyper::body::Payload\">Payload</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: H2Exec&lt;&lt;S::Service as <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&gt;::<a class=\"type\" href=\"hyper/service/trait.Service.html#associatedtype.Future\" title=\"type hyper::service::Service::Future\">Future</a>, B&gt;,&nbsp;</span>",synthetic:false,types:["hyper::server::conn::Serve"]},{text:"impl <a class=\"trait\" href=\"hyper/rt/trait.Stream.html\" title=\"trait hyper::rt::Stream\">Stream</a> for <a class=\"struct\" href=\"hyper/server/conn/struct.AddrIncoming.html\" title=\"struct hyper::server::conn::AddrIncoming\">AddrIncoming</a>",synthetic:false,types:["hyper::server::tcp::AddrIncoming"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()