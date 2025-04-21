BytChat
=====

The goal is to create a distributed messaging system

Thoughts
----

- Lets try a kademlia DHT over Chord to keep the DHT more dynamic and allow for growth
- We will want to switch to a 160 bit table over the current model to keep it more secure
- We can allow users to generate UIDs based off of RSA Key HASH and published to DHT

- We can create a system similar to BitTorrent Tracking for the endpoints to allow for more than 1 device per UID.
- We can ensure syncing based off of challenges

- We can store messages on tracker for a TTL lets give it 3 - 7 days

- We can have a meta option within the DHT for users to store simple information like users icon and name
