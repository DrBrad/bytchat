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


User
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


User Meta
====

```json
{
    "id": "UID",
    "content": {
        "name": "Edward",
        "avatar": BYTES,
        "timestamp": long
    },
    "signature": SIGNATURE_OF_CONTENT
}
```

A user can create meta at any point, it will contain the avatar and nam of the user, possibly more later on. To update this information or create it the user must sign the meta with their private key. The nodes hosting this information will use the UID given within the meta and verify the integrity of the meta. If the checks are valid the meta is stored for a TTL of 3-7 days. The meta will be stored on the 5 closest nodes to the users sha("meta" + UID).

If a user needs to obtain the meta for a particular UID they will take that UID sha("meta" + UID) and find the 5 closest nodes to this. Once they have found them they will ask for the meta in which they will then verify the integrity 
via the signature based off of the public key of that UID.

Message Handshake
====

```json
{
    "to": "UID",
    "content": {
        "from": "UID",
        "public_key": RSA_PUBLIC_KEY
    },
    "timestamp": long,
    "signature": SIGNATURE_OF_CONTENT
}
```

When a user wishes to start messaging a new person they will lookup the RSA for that UID and find the 5 nodes closest to sha("chat" + UID). Once they have found those 5 nodes they will then generate a handshake like the one above, once the user has received the message they will send a similar message as the one above back to the sender using the same process. When both parties have the others public key they will use each others public key as diffie-helmen password which will be used for messaging later on.





Messaging Transit
====

```json
{
    "id": "UID",
    "to": "UID",
    "content": AES-512-CBC {
        "mime_type": "image/jpeg",
        "data": BYTES
    },
    "timestamp": long,
    "signature": SIGNATURE_OF_CONTENT
}
```

Once a handshake has been made users can send messages via the sha("chat" + UID) and sending messages like the example above. Once a device has received the message they will send back to the tracker that they have received the message by sending the signature and signing it with their own signature. In which case the tracker will remove the message. If a tracker has held onto the message for more than a TTL of 3-7 days it will remove the message.

- delivered should be message signature + signature sent to the messages from tracker.



Message Syncing
====

Assuming the user wants more than 1 device to send messages from.















DataBase Store
====

- We will want to store meta being name, avatar, RSA, UID, etc...
- Contacts
- Chats which will store Chat UIDs, Keys, and Users
- 


















Alternate Plane
====

We will only store public / private keys locally along with meta

This will be the new requests

| Key          | Val                                     |
|--------------|-----------------------------------------|
| bind_tracker | Binds to a tracker                      |
| find_user    | Get list of nodes for user from tracker |

- The DHT will only be user for routing in other words

We will then have a seperate socket in which message requests and messages are sent through.

Nodes will have to do finds on themselves with refreshes to ensure they have an updated list and new nodes should announce to each node given from the tracker.
When a node announces they will then sync messages up between each other after verifying they are the same - nodes bound to the same ID should be the same user.


Bind Tracker Request
----

- we need to ensure replay attacks cannot occur...
- though it wouldn't be the end of the world as without the correct key they cannot decode, but they could eclipse messages to the receiver...

```json
{
    "t": "aa",
    "y": "r",
    "r": {
        "id": "mnopqrstuvwxyz123456",
        "key": "mnopqrstuvwxyz123456mnopqrstuvwxyz123456",
        "port": 6771,
        "signature": "mnopqrstuvwxyz123456",
        "timestamp": 12736172123
    }
}
```
