(function() {var implementors = {};
implementors["tracing_opentelemetry"] = [{text:"impl&lt;S, T&gt; <a class=\"trait\" href=\"tracing_subscriber/layer/trait.Layer.html\" title=\"trait tracing_subscriber::layer::Layer\">Layer</a>&lt;S&gt; for <a class=\"struct\" href=\"tracing_opentelemetry/struct.OpentelemetryLayer.html\" title=\"struct tracing_opentelemetry::OpentelemetryLayer\">OpentelemetryLayer</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tracing_core/subscriber/trait.Subscriber.html\" title=\"trait tracing_core::subscriber::Subscriber\">Subscriber</a> + for&lt;'span&gt; <a class=\"trait\" href=\"tracing_subscriber/registry/trait.LookupSpan.html\" title=\"trait tracing_subscriber::registry::LookupSpan\">LookupSpan</a>&lt;'span&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"opentelemetry/api/trace/tracer/trait.Tracer.html\" title=\"trait opentelemetry::api::trace::tracer::Tracer\">Tracer</a> + 'static,&nbsp;</span>",synthetic:false,types:["tracing_opentelemetry::layer::OpentelemetryLayer"]},];
implementors["tracing_subscriber"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()