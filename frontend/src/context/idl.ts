export type Solciv = {
  "version": "0.1.0",
  "name": "solciv",
  "instructions": [
    {
      "name": "initializeGame",
      "accounts": [
        {
          "name": "game",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "map",
          "type": {
            "array": [
              "u8",
              400
            ]
          }
        }
      ]
    },
    {
      "name": "initializePlayer",
      "accounts": [
        {
          "name": "game",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveUnit",
      "accounts": [
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "unitId",
          "type": "u8"
        },
        {
          "name": "x",
          "type": "u8"
        },
        {
          "name": "y",
          "type": "u8"
        }
      ]
    },
    {
      "name": "endTurn",
      "accounts": [
        {
          "name": "game",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Game",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "turn",
            "type": "u32"
          },
          {
            "name": "map",
            "type": {
              "array": [
                "u8",
                400
              ]
            }
          }
        ]
      }
    },
    {
      "name": "Player",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "game",
            "type": "publicKey"
          },
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "points",
            "type": "u64"
          },
          {
            "name": "units",
            "type": {
              "vec": {
                "defined": "Unit"
              }
            }
          },
          {
            "name": "resources",
            "type": {
              "defined": "Resources"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Unit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unitId",
            "type": "u8"
          },
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "game",
            "type": "publicKey"
          },
          {
            "name": "unitType",
            "type": {
              "defined": "UnitType"
            }
          },
          {
            "name": "x",
            "type": "u8"
          },
          {
            "name": "y",
            "type": "u8"
          },
          {
            "name": "attack",
            "type": "u8"
          },
          {
            "name": "health",
            "type": "u8"
          },
          {
            "name": "movementRange",
            "type": "u8"
          },
          {
            "name": "remainingActions",
            "type": "u8"
          },
          {
            "name": "isAlive",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "Resources",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gold",
            "type": "u32"
          },
          {
            "name": "food",
            "type": "u32"
          },
          {
            "name": "wood",
            "type": "u32"
          },
          {
            "name": "stone",
            "type": "u32"
          },
          {
            "name": "iron",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "UnitType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Settler"
          },
          {
            "name": "Builder"
          },
          {
            "name": "Warrior"
          },
          {
            "name": "Archer"
          },
          {
            "name": "Swordsman"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnitNotFound",
      "msg": "Unit with given ID not found"
    },
    {
      "code": 6001,
      "name": "CannotMove",
      "msg": "Unit cannot move this turn"
    },
    {
      "code": 6002,
      "name": "OutOfMovementRange",
      "msg": "Out of movement range"
    },
    {
      "code": 6003,
      "name": "OutOfMapBounds",
      "msg": "Out of map bounds"
    },
    {
      "code": 6004,
      "name": "TileOccupied",
      "msg": "Tile is occupied by another unit"
    }
  ],
  "metadata": {
    "address": "GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz"
  }
};

export const IDL: Solciv = {
  "version": "0.1.0",
  "name": "solciv",
  "instructions": [
    {
      "name": "initializeGame",
      "accounts": [
        {
          "name": "game",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "map",
          "type": {
            "array": [
              "u8",
              400
            ]
          }
        }
      ]
    },
    {
      "name": "initializePlayer",
      "accounts": [
        {
          "name": "game",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveUnit",
      "accounts": [
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "unitId",
          "type": "u8"
        },
        {
          "name": "x",
          "type": "u8"
        },
        {
          "name": "y",
          "type": "u8"
        }
      ]
    },
    {
      "name": "endTurn",
      "accounts": [
        {
          "name": "game",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "playerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "player",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Game",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "turn",
            "type": "u32"
          },
          {
            "name": "map",
            "type": {
              "array": [
                "u8",
                400
              ]
            }
          }
        ]
      }
    },
    {
      "name": "Player",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "game",
            "type": "publicKey"
          },
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "points",
            "type": "u64"
          },
          {
            "name": "units",
            "type": {
              "vec": {
                "defined": "Unit"
              }
            }
          },
          {
            "name": "resources",
            "type": {
              "defined": "Resources"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Unit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unitId",
            "type": "u8"
          },
          {
            "name": "player",
            "type": "publicKey"
          },
          {
            "name": "game",
            "type": "publicKey"
          },
          {
            "name": "unitType",
            "type": {
              "defined": "UnitType"
            }
          },
          {
            "name": "x",
            "type": "u8"
          },
          {
            "name": "y",
            "type": "u8"
          },
          {
            "name": "attack",
            "type": "u8"
          },
          {
            "name": "health",
            "type": "u8"
          },
          {
            "name": "movementRange",
            "type": "u8"
          },
          {
            "name": "remainingActions",
            "type": "u8"
          },
          {
            "name": "isAlive",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "Resources",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gold",
            "type": "u32"
          },
          {
            "name": "food",
            "type": "u32"
          },
          {
            "name": "wood",
            "type": "u32"
          },
          {
            "name": "stone",
            "type": "u32"
          },
          {
            "name": "iron",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "UnitType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Settler"
          },
          {
            "name": "Builder"
          },
          {
            "name": "Warrior"
          },
          {
            "name": "Archer"
          },
          {
            "name": "Swordsman"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnitNotFound",
      "msg": "Unit with given ID not found"
    },
    {
      "code": 6001,
      "name": "CannotMove",
      "msg": "Unit cannot move this turn"
    },
    {
      "code": 6002,
      "name": "OutOfMovementRange",
      "msg": "Out of movement range"
    },
    {
      "code": 6003,
      "name": "OutOfMapBounds",
      "msg": "Out of map bounds"
    },
    {
      "code": 6004,
      "name": "TileOccupied",
      "msg": "Tile is occupied by another unit"
    }
  ],
  "metadata": {
    "address": "GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz"
  }
};