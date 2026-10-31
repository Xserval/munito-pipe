1- Choices of crates for the HTTP requests and dealing with concurrency and async :
- reqwest: async, ergonomic, full-featured
- ureq: blocking/sync, fast compile times, small binary, correct for CLI tools and scripts
- hyper: low-level HTTP building block, not an end-user client; use reqwest which wraps it

I need a basic API request, hyper is much too low level and work for that as I don't need that much control over the HTTP Infrastructure.
Ureq is good but I do need async because it needs to be able to stream the response token by token, and send many requests concurrently later and I don't need to be blocking any I/O
So reqwest is good.

To use reqwest which I need, I need to depend on Tokio on this project

2-Choices for tokio, use multithread or single
To be safe and as multi thread is the default of the library I sticked to multi thread, especially since that would relieve the CPU in case of heavy calls from my RAG project.

3- Error handling
Because we're using async and await? a lot, we need main to return error, after looking there's multiple ways but three populars are :
- Box<dyn Error> in the signature: no extra crate, fine for a small program.
- anyhow: a crate that makes this more comfortable, popular for applications.
- thiserror: for defining your own error types, which libraries use so callers can react to specific errors.

For now, for testing reason and before industrializing it, Box<dyn Error> is what will be implemented, further down, and after researching on it we'll probably move to anyhow

4- Typed structs instead of serde_json::Value for the response

At first I parsed the response into a Value and dug into it with ["choices"][0]["message"]["content"]. It worked, but it hid a problem: I forgot that "choices" is a list, and instead of an error I got "null" printed in my output. Nothing told me my path was wrong.

The format of the response is documented and stable, so I switched to structs that mirror the JSON (Message, Choice, Usage, JsonAnswer) and let serde fill them in. Now a wrong field name is a compile error, and if the API sends something unexpected, parsing fails with a message that names the field.

I made content and reasoning Option<String> because a model can answer without one of them. With a plain String, a missing field would make the whole parse fail. With Option, the parse works and I decide what to do when the value is None.

The downside: if the API changes its format, parsing can fail where the Value version would have kept going. For a client library I prefer a clear error over a silent null.
