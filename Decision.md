1- Choices of crates for the HTTP requests and dealing with concurrency and async :
- reqwest: async, ergonomic, full-featured
- ureq: blocking/sync, fast compile times, small binary, correct for CLI tools and scripts
- hyper: low-level HTTP building block, not an end-user client; use reqwest which wraps it

I need a basic API request, hyper is much too low level and work for that as I don't need that much control over the HTTP Infrastructure.
Ureq is good but I do need async because it needs to be able to stream the response token by token, and send many requests concurrently later and I don't need to be blocking any I/O
So reqwest is good.

To use reqwest which I need, I need to implement Tokio on this project

2-Choices for tokio, use multithread or single
To be safe and as multi thread is the default of the library I sticked to multi thread, especially since that would relieve the CPU in case of heavy calls from my RAG project.

3- Error handling
Because we're using async and await? a lot, we need main to return error, after looking there's multiple ways but three populars are :
- Box<dyn Error> in the signature: no extra crate, fine for a small program.
- anyhow: a crate that makes this more comfortable, popular for applications.
- thiserror: for defining your own error types, which libraries use so callers can react to specific errors.

For now, for testing reason and before industrializing it, Box<dyn Error> is what will be implemented, further down, and after researching on it we'll probably move to anyhow
