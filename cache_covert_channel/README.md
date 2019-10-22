Cache Covert Channel
--------------------

Covert channel communication using CPU caches.

Original cache-based side channel attack: https://github.com/polymorf/misc-cache-attacks

Testing environment
-------------------

CPU : Intel i7 6700-HQ (laptop).
RAM : 16 GB.
OS  : Fedora.

Status
------

Can transmit text data between two processes on the same machine.

Not reliable and poor results so far (only partial recovery of the message at
best).
