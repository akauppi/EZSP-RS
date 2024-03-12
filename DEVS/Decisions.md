# Decisions

Brief background on why certain libraries got selected; which didn't, and why not.



## Async library

Needed some help (higher abstractions) for the logic. Looked around:

- `std::sync::mpsc`

	Is not async/await. We don't want blocking receiving of messages, but an async "until anything is available".

- [`async-channel`](https://crates.io/crates/async-channel)

	Looks to have fair user base.
	
	Tried it, I think but moved on to `actix`.

<!--
	>An async multi-producer multi-consumer channel, where each message can be received by only one of all existing consumers.

	<p />

	>A channel has the Sender and Receiver side. Both sides are cloneable and can be shared among multiple threads.
-->

- [`actix`](https://actix.rs/docs/actix)

	Actors, in Rust.
	
	Seems to provide the `async`/`await` interface we need for handling the ASH async messages. Runs by default in a single thread (not essential, but may help fitting this to an ESP32 if it comes to that).
	
	Actix documentation themselves states that they are now more focused on the web framework, and that..
	
	>[the] usefulness [of the `actix` actors] as a general tool is diminishing as the futures and async/await ecosystem matures. 
	
	Not sure whether this means I *could* code the stuff without library help. Maybe later, once more black-belted in Rust.

	<!-- #whisper
	In particular, need to be able to time futures.
	-->
