# Decisions

Brief background on why certain libraries got selected; which didn't, and why not.

## Channel libraries

## `std::sync::mpsc`

..is synchronous. We don't want blocking receiving of messages, but an async "until anything is available" API.

Looking further.

## [`async-channel`](https://crates.io/crates/async-channel)

Next in line. Looks to have fair user base.

>An async multi-producer multi-consumer channel, where each message can be received by only one of all existing consumers.

<p />

>A channel has the Sender and Receiver side. Both sides are cloneable and can be shared among multiple threads.

