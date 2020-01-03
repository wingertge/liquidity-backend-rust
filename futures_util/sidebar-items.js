initSidebarItems({"macro":[["join","Polls multiple futures simultaneously, returning a tuple of all results once complete."],["pending","A macro which yields to the event loop once."],["pin_mut","Pins a value on the stack."],["poll","A macro which returns the result of polling a future once within the current `async` context."],["ready","Extracts the successful type of a `Poll<T>`."],["select","Polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime. Futures directly passed to `select!` must be `Unpin` and implement `FusedFuture`."],["try_join","Polls multiple futures simultaneously, resolving to a [`Result`] containing either a tuple of the successful outputs or an error."]],"mod":[["compat","Futures 0.1 / 0.3 shims"],["future","Futures"],["io","IO"],["lock","Futures-powered synchronization primitives."],["never","Definition of the `Never` type, a stand-in for the `!` type until it becomes stable."],["sink","Sinks"],["stream","Streams"],["task","Task notification"]]});