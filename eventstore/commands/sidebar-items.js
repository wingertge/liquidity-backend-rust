initSidebarItems({"struct":[["AllCatchupSubscribe","Like `RegularCatchupSubscribe` but specific to the system stream '$all'."],["ConnectToPersistentSubscription","A subscription model where the server remembers the state of the consumption of a stream. This allows for many different modes of operations compared to a regular subscription where the client hols the subscription state."],["CreatePersistentSubscription","A command that creates a persistent subscription for a given group."],["DeletePersistentSubscription","Command that  deletes a persistent subscription."],["DeleteStream","Command that deletes a stream. More information on [Deleting stream and events]."],["ReadAllEvents","Like `ReadStreamEvents` but specialized to system stream '$all'."],["ReadEvent","Command that reads an event from a given stream."],["ReadStreamEvents","A command that reads several events from a stream. It can read events forward or backward."],["ReadStreamMetadata","Reads a stream metadata command."],["RegularCatchupSubscribe","Subscribes to a given stream. This kind of subscription specifies a starting point (by default, the beginning of a stream). For a regular stream, that starting point will be an event number. For the system stream `$all`, it will be a position in the transaction file (see `subscribe_to_all_from`). This subscription will fetch every event until the end of the stream, then will dispatch subsequently written events."],["SubscribeToStream","Represents a volatile subscription. For example, if a stream has 100 events in it when a subscriber connects, the subscriber can expect to see event number 101 onwards until the time the subscription is closed or dropped."],["Transaction","Represents a multi-requests transaction with the GetEventStore server."],["TransactionStart","Command that starts a transaction on a stream."],["UpdatePersistentSubscription","Command that updates an already existing subscription's settings."],["WriteEvents","Command that sends events to a given stream."],["WriteStreamMetadata","Write stream metadata command."]]});