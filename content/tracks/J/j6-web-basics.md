---
id: j6-web-basics
title: How the web fits together
type: section
track: J
order: 60
verified: 2026-08-02
volatility: low
answer: >
  One computer asks and another answers, one request at a time, over HTTP
  (Hypertext Transfer Protocol). The response carries a status code, and the
  first digit is the whole story: 4 means your request was wrong, 5 means the
  server broke.
owns:
  - HTTP
  - client and server
  - request and response
  - API
  - REST
  - status codes
see_also:
  - c6-ports-and-localhost
  - j2-the-config-formats-nobody-explains
  - i2-servers-and-hosting
  - json
  - f1-how-to-read-an-error-message
  - i1-what-deployment-means
keywords:
  - http
  - status code
  - "404"
  - 500 error
  - rest api
  - cors
  - request response
  - endpoint
---

## More

Every web thing is the same shape underneath. One program asks, another answers, and then
the connection ends. The asker is the **client**, usually a browser. The answerer is the
**server**. The conversation happens over HTTP (Hypertext Transfer Protocol), which is a
format for those two messages and nothing more.

A **request** has four parts:

- A **method**, saying what kind of asking this is. `GET` fetches, `POST` submits.
- A **path**, saying what you want: `/users/42`.
- **Headers**, which are settings for this one request, including who you are.
- Sometimes a **body**, which is the data you are sending.

A **response** has three:

- A **status code**, a three-digit number saying how it went.
- **Headers**, including what kind of data is coming back.
- A **body**, usually HTML (Hypertext Markup Language) for a page or JSON (JavaScript Object
  Notation) for data.

The status code is the part worth memorizing, and only the first digit matters:

| Starts with | Means | What you do |
|---|---|---|
| 2 | It worked | Nothing |
| 3 | It moved | Nothing, the browser follows it |
| 4 | Your request was wrong | Fix the request |
| 5 | The server broke while handling it | Read the server's logs |

That 4 versus 5 split is the single most useful thing on this card. A 4 means the problem is
in what you sent: a bad address, a missing key, a token that expired. The server is fine and
its logs will tell you nothing interesting. A 5 means the server hit an error while trying,
which is a bug on its side, and the actual error is in its logs rather than in your browser
([f4](#f4-logs)).

An **API (Application Programming Interface)** is the same machinery with the human page
left off: an address you can send a request to that answers with data instead of a web page.
When someone says "call the API," they mean send an HTTP request to a specific path and read
the JSON that comes back.

## Full

### A request and a response, annotated

```text
POST /api/users HTTP/1.1
Host: example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOi...

{"email":"nyx@example.com","name":"Nyx"}
```

Method, path, and version on the first line. Headers next, one per line. A blank line. Then
the body, which here is JSON.

```text
201 Created
Content-Type: application/json

{"id":42,"email":"nyx@example.com"}
```

Status code first, headers, blank line, body. Every web interaction you ever have is this,
repeated.

### The methods

| Method | Means | Safe to repeat |
|---|---|---|
| `GET` | Give me this. Never changes anything | Yes |
| `POST` | Here is something new | No, twice creates two |
| `PUT` | Replace this with what I am sending | Yes |
| `PATCH` | Change these fields of this | Usually |
| `DELETE` | Remove this | Yes, the second one just finds nothing |

A `GET` that changes data is a design mistake with real consequences, because browsers,
proxies, and preview tools all assume they can repeat one freely.

### The status codes you will actually meet

- **200 OK.** It worked.
- **201 Created.** It worked and something new exists.
- **204 No Content.** It worked and there is nothing to send back. Common after a delete.
- **301 and 302.** Moved. The response carries the new address and the browser follows it.
- **400 Bad Request.** The server could not make sense of what you sent. Usually malformed
  JSON or a missing field.
- **401 Unauthorized.** You are not signed in, or your token expired. The name is wrong: it
  means unauthenticated.
- **403 Forbidden.** The server knows exactly who you are and you are not allowed. `401`
  means log in, `403` means stop asking.
- **404 Not Found.** No such path. Either the address is wrong or the thing was deleted.
- **409 Conflict.** It clashes with something that already exists, like a duplicate email.
- **422 Unprocessable Content.** The shape was fine and the values were not. Validation
  failures land here.
- **429 Too Many Requests.** You are being rate limited. Wait, and read the `Retry-After`
  header.
- **500 Internal Server Error.** The server threw an exception. There is a stack trace in
  its logs with your name on it ([f2](#f2-stack-traces)).
- **502 and 504.** A server in front of your program could not reach it or waited too long.
  Usually your program crashed on startup or is too slow.

### What RESTful means, at recognition level

REST (Representational State Transfer) is a convention for laying out an API, not a
technology. The convention is that paths name things and methods say what to do with them:

```text
GET    /users        list them
POST   /users        create one
GET    /users/42     fetch that one
PATCH  /users/42     change part of that one
DELETE /users/42     remove it
```

Nouns in the path, verbs in the method. When somebody calls an API RESTful, that is the
whole claim. Plenty of working services follow the shape loosely, and the shape is a
convention rather than a rule anyone enforces.

The main alternative you will see named is GraphQL, where there is one path and the request
body describes exactly which fields you want ([GraphQL](#graphql)).

### The headers that come up

- `Content-Type` says what format the body is in. `application/json` is the common one, and
  a server that expected JSON and received a form will answer 400.
- `Authorization` carries your credentials, usually as `Bearer <token>`. This is a secret and
  belongs nowhere near a commit or a chat window ([g8](#g8-what-never-to-paste-into-a-chat)).
- `Set-Cookie` and `Cookie` are how a browser keeps you signed in across requests, since
  HTTP itself remembers nothing between them.

### The one that will confuse you

CORS (Cross-Origin Resource Sharing) is a browser rule. A page loaded from one address is
not allowed to read a response from a different address unless that other server explicitly
says it is fine. The error appears in the browser console and reads something like "blocked
by CORS policy."

Two things about it that save hours. It is enforced by the browser only, so the same request
from a terminal works, and that working request proves nothing. And it is fixed on the
**server**, by adding a header naming the origins it allows. No amount of editing the front
end fixes a CORS error.

You meet it during development because your front end runs on one port and your back end on
another, which counts as a different origin ([c6](#c6-ports-and-localhost)).

### Where to look when something fails

Open the browser's developer tools with `F12` and go to the Network tab. Every request the
page made is listed with its status code, and clicking one shows the exact headers and body
that went out and came back. That panel answers "did the request even go out," "what
exactly did the server say," and "which request was the failing one," and it is faster than
any amount of reading the code.

The address bar's `https` prefix means the whole conversation is encrypted using TLS (Transport Layer Security)
(Transport Layer Security), so nobody between you and the server can read it. Certificates
and how a site gets one are in [i2](#i2-servers-and-hosting).
