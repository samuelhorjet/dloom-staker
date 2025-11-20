/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/dloom_staker.json`.
 */
export type DloomStaker = {
  "address": "D67Cj1mwDuJZKJ9DW9MksWAXZQk3hh7nB6AGzcf6hkph",
  "metadata": {
    "name": "dloomStaker",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addLockupTier",
      "discriminator": [
        101,
        141,
        154,
        87,
        73,
        58,
        66,
        7
      ],
      "accounts": [
        {
          "name": "farm",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "farm"
          ]
        }
      ],
      "args": [
        {
          "name": "duration",
          "type": "i64"
        },
        {
          "name": "multiplier",
          "type": "u16"
        }
      ]
    },
    {
      "name": "claimRewards",
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "staker"
          ]
        },
        {
          "name": "farm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  97,
                  114,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "farm.lp_mint",
                "account": "farm"
              },
              {
                "kind": "account",
                "path": "farm.reward_mint",
                "account": "farm"
              }
            ]
          },
          "relations": [
            "staker"
          ]
        },
        {
          "name": "rewardMint"
        },
        {
          "name": "staker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "rewardVault",
          "writable": true
        },
        {
          "name": "userRewardTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "rewardMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "compound",
      "discriminator": [
        165,
        208,
        251,
        78,
        242,
        160,
        141,
        47
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "staker"
          ]
        },
        {
          "name": "farm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  97,
                  114,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "farm.lp_mint",
                "account": "farm"
              },
              {
                "kind": "account",
                "path": "farm.reward_mint",
                "account": "farm"
              }
            ]
          },
          "relations": [
            "staker"
          ]
        },
        {
          "name": "staker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "rewardVault",
          "writable": true
        },
        {
          "name": "lpVault",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "createFarm",
      "discriminator": [
        74,
        59,
        128,
        160,
        87,
        174,
        153,
        194
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "farm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  97,
                  114,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "lpMint"
              },
              {
                "kind": "account",
                "path": "rewardMint"
              }
            ]
          }
        },
        {
          "name": "lpMint"
        },
        {
          "name": "lpVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  112,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "rewardMint"
        },
        {
          "name": "rewardVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "farm"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "rewardMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "fundFarm",
      "discriminator": [
        24,
        200,
        152,
        129,
        117,
        142,
        212,
        252
      ],
      "accounts": [
        {
          "name": "farm",
          "writable": true
        },
        {
          "name": "rewardVault",
          "writable": true
        },
        {
          "name": "funder",
          "writable": true,
          "signer": true
        },
        {
          "name": "funderTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeStaker",
      "discriminator": [
        131,
        155,
        29,
        159,
        5,
        65,
        156,
        247
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "farm"
        },
        {
          "name": "staker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "setRewardRate",
      "discriminator": [
        253,
        201,
        190,
        20,
        48,
        38,
        120,
        34
      ],
      "accounts": [
        {
          "name": "farm",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "farm"
          ]
        }
      ],
      "args": [
        {
          "name": "newRate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "stake",
      "discriminator": [
        206,
        176,
        202,
        18,
        200,
        209,
        179,
        108
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "staker"
          ]
        },
        {
          "name": "farm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  97,
                  114,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "farm.lp_mint",
                "account": "farm"
              },
              {
                "kind": "account",
                "path": "farm.reward_mint",
                "account": "farm"
              }
            ]
          },
          "relations": [
            "staker"
          ]
        },
        {
          "name": "staker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "lpVault",
          "writable": true
        },
        {
          "name": "userLpTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "duration",
          "type": "i64"
        }
      ]
    },
    {
      "name": "unstake",
      "discriminator": [
        90,
        95,
        107,
        42,
        205,
        124,
        50,
        225
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "staker"
          ]
        },
        {
          "name": "farm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  97,
                  114,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "farm.lp_mint",
                "account": "farm"
              },
              {
                "kind": "account",
                "path": "farm.reward_mint",
                "account": "farm"
              }
            ]
          },
          "relations": [
            "staker"
          ]
        },
        {
          "name": "staker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "farm"
              }
            ]
          }
        },
        {
          "name": "lpVault",
          "writable": true
        },
        {
          "name": "userLpTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "farm",
      "discriminator": [
        161,
        156,
        211,
        253,
        250,
        64,
        53,
        250
      ]
    },
    {
      "name": "staker",
      "discriminator": [
        171,
        229,
        193,
        85,
        67,
        177,
        151,
        4
      ]
    }
  ],
  "events": [
    {
      "name": "compounded",
      "discriminator": [
        187,
        212,
        188,
        187,
        183,
        29,
        107,
        152
      ]
    },
    {
      "name": "farmCreated",
      "discriminator": [
        161,
        208,
        67,
        228,
        191,
        208,
        35,
        143
      ]
    },
    {
      "name": "farmFunded",
      "discriminator": [
        209,
        47,
        70,
        106,
        159,
        59,
        116,
        247
      ]
    },
    {
      "name": "lockupTierAdded",
      "discriminator": [
        99,
        186,
        251,
        124,
        128,
        133,
        220,
        2
      ]
    },
    {
      "name": "rewardRateUpdated",
      "discriminator": [
        176,
        128,
        176,
        106,
        40,
        165,
        210,
        144
      ]
    },
    {
      "name": "rewardsClaimed",
      "discriminator": [
        75,
        98,
        88,
        18,
        219,
        112,
        88,
        121
      ]
    },
    {
      "name": "staked",
      "discriminator": [
        11,
        146,
        45,
        205,
        230,
        58,
        213,
        240
      ]
    },
    {
      "name": "stakerInitialized",
      "discriminator": [
        68,
        182,
        206,
        69,
        123,
        190,
        76,
        198
      ]
    },
    {
      "name": "tokensBurned",
      "discriminator": [
        230,
        255,
        34,
        113,
        226,
        53,
        227,
        9
      ]
    },
    {
      "name": "unstaked",
      "discriminator": [
        27,
        179,
        156,
        215,
        47,
        71,
        195,
        7
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "stakeLocked",
      "msg": "Cannot unstake, the stake is still locked."
    },
    {
      "code": 6001,
      "name": "tierNotFound",
      "msg": "The specified lock-up duration is not available for this farm."
    },
    {
      "code": 6002,
      "name": "cannotChangeLockup",
      "msg": "The lock-up duration cannot be changed while a stake is active."
    },
    {
      "code": 6003,
      "name": "stakerAlreadyInitialized",
      "msg": "A staker account can only be initialized once."
    },
    {
      "code": 6004,
      "name": "zeroAmount",
      "msg": "The amount must be greater than zero."
    },
    {
      "code": 6005,
      "name": "insufficientBalance",
      "msg": "Insufficient balance to perform this action."
    },
    {
      "code": 6006,
      "name": "mathOverflow",
      "msg": "A math operation resulted in an overflow or underflow."
    },
    {
      "code": 6007,
      "name": "invalidRewardRate",
      "msg": "The specified reward rate is invalid."
    },
    {
      "code": 6008,
      "name": "compoundingNotSupported",
      "msg": "This farm does not support compounding because the reward token is not the same as the LP token."
    },
    {
      "code": 6009,
      "name": "noRewardsToClaim",
      "msg": "There are no rewards to claim at this time."
    },
    {
      "code": 6010,
      "name": "duplicateLockupTier",
      "msg": "This lock-up tier already exists."
    }
  ],
  "types": [
    {
      "name": "compounded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "rewardAmountCompounded",
            "type": "u64"
          },
          {
            "name": "newTotalStakedBalance",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "farm",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "lpMint",
            "type": "pubkey"
          },
          {
            "name": "lpVault",
            "type": "pubkey"
          },
          {
            "name": "rewardMint",
            "type": "pubkey"
          },
          {
            "name": "rewardVault",
            "type": "pubkey"
          },
          {
            "name": "rewardRate",
            "type": "u64"
          },
          {
            "name": "lastUpdateTimestamp",
            "type": "i64"
          },
          {
            "name": "totalWeightedStake",
            "type": "u128"
          },
          {
            "name": "rewardPerTokenStored",
            "type": "u128"
          },
          {
            "name": "lockupTiers",
            "type": {
              "vec": {
                "defined": {
                  "name": "lockupTier"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "farmCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "lpMint",
            "type": "pubkey"
          },
          {
            "name": "rewardMint",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "farmFunded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "funder",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "lockupTier",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "duration",
            "type": "i64"
          },
          {
            "name": "multiplier",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "lockupTierAdded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "tier",
            "type": {
              "defined": {
                "name": "lockupTier"
              }
            }
          }
        ]
      }
    },
    {
      "name": "rewardRateUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "newRate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rewardsClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "staked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "lockupEndTimestamp",
            "type": "i64"
          },
          {
            "name": "totalStakedBalance",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "staker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "farm",
            "type": "pubkey"
          },
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "lockupEndTimestamp",
            "type": "i64"
          },
          {
            "name": "rewardMultiplier",
            "type": "u16"
          },
          {
            "name": "rewardsPaid",
            "type": "u128"
          },
          {
            "name": "rewardPerTokenSnapshot",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "stakerInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakerAddress",
            "type": "pubkey"
          },
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "tokensBurned",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "unstaked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "farmAddress",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "totalStakedBalance",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
