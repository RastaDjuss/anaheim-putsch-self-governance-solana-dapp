/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/anaheim.json`.
 */
export type Anaheim = {
  "address": "",
  "metadata": {
    "name": "anaheim",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Solan Governance Dapp and social media for powerr to the people nextjs, react, typescript and anchor used"
  },
  "instructions": [
    {
      "name": "closePost",
      "discriminator": [
        131,
        190,
        34,
        94,
        190,
        71,
        183,
        81
      ],
      "accounts": [
        {
          "name": "post",
          "writable": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "createPost",
      "discriminator": [
        123,
        92,
        184,
        29,
        231,
        24,
        15,
        202
      ],
      "accounts": [
        {
          "name": "post",
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "postAccount",
      "discriminator": [
        85,
        236,
        139,
        84,
        240,
        243,
        196,
        23
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidContent",
      "msg": "The provided content is invalid or empty."
    },
    {
      "code": 6001,
      "name": "contentTooLong",
      "msg": "The content length exceeds the maximum allowed."
    },
    {
      "code": 6002,
      "name": "usernameTooLong",
      "msg": "The username length exceeds the maximum allowed."
    },
    {
      "code": 6003,
      "name": "invalidUsername",
      "msg": "Username is invalid."
    }
  ],
  "types": [
    {
      "name": "postAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "content",
            "type": "string"
          },
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "createdAt",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
