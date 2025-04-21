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


Example of User
====

| Key        | Val   | Type        |
|------------|-------|-------------|
| public_Key | bytes | RSA_PUB_KEY |

Assume Creation
----
When the user starts they will join the DHT and generate an RSA Public Key. They will then hash this public key via SHA-1 (Update to SHA-512 later on). With the hash they will then send it to the 5 nodes closest to that Hash. This hash will be known as the users UID, and will be used for messaging later on. They will then create a socket with the 5 nodes closest to their UID and hang.

Assume Joining with pre-existing UID
----
In this case the user will hash their public key (they should be aware of the public and private key). They will then find the 5 nodes closest to that hash and create a socket that will hang.


Example of user Meta
====

| Key        | Val     | Type        |
|------------|---------|-------------|
| uid        | id      | UID         |
| avatar     | image   | bytes       |
| name       | Edward  | string      |
| signature  | bytes   | SIGNATURE   |

A user can create meta at any point, it will contain the avatar and nam of the user, possibly more later on. To update this information or create it the user must sign the meta with their private key. The nodes hosting this information will use the UID given within the meta and verify the integrity of the meta. If the checks are valid the meta is stored for a TTL of 3-7 days. The meta will be stored on the 5 closest nodes to the users sha("meta" + UID).

If a user needs to obtain the meta for a particular UID they will take that UID sha("meta" + UID) and find the 5 closest nodes to this. Once they have found them they will ask for the meta in which they will then verify the integrity 
via the signature based off of the public key of that UID.




Example of message store - not representation of message transit
====

| Key          | Val            | Type  |
|--------------|----------------|-------|
| type         | to/from        | enum  |
| chat         | id             | UID   |
| message      | bytes          | bytes |
| content_type | image          | enum  |
| timestamp    | date           | long  |
| status       | sent/delivered | enum  |

