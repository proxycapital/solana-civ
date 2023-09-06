### Solana Civ

Fully on-chain proof of concept inspired by Sid Meier's Civilization.
In SolanaCiv, players use smart contracts to manage their civilization by building structures, recruiting workers and units, and fighting off barbarians.
Players earn resources (SPL tokens) like gold, food, and lumber through smart contract that simulate resource generation. The game is turn-based, with each turn representing 1 day. Players can perform many actions (transactions) per turn: move units, build, fight, upgrade and more.

### Game design

#### Initialization
- When a player joins, frontend generates a burner wallet and funds it with 1 devnet SOL
- Smart contract initializes the civilization with a starting amount of $GOLD, $FOOD, $LUMBER, 1 worker and 1 scout. Assets are soulbound to a game session and cannot be transferred to a different game session or wallet.
- One city is automatically built for each civilization upon joining.

#### Turn mechanics
- At the start of each turn, players receive (or lose) resources based on their current buildings and units to maintain.
- Players can then choose actions for their workers, soldiers and cities. Every action is an on-chain transaction that triggers the smart contract. Players can: construct a building, recruit units, move units on the global map, send soldiers to fight barbarians.

#### Construction
- Building a structure costs $LUMBER and/or $GOLD. Some structures can be built only by workers on the global map, other directly in the city.
- Construction time is constant for each structure and depends on number of turns.

#### Resource gathering
- $GOLD, $FOOD, and $LUMBER generation occurs at the beginning of each turn, affected by the number and type of buildings and units.

#### Tokens
- 🪙 $GOLD - currency for trading, building and maintenance.
- 🌽 $FOOD - needed for recruiting.
- 🪵 $LUMBER - required for constructing buildings.

#### Units
| Image | Unit | Movement | Melee | HP | Cost | Maintenance |
|---|---|---|---|---|---|---|
|  | Worker | 2 | 0 | ❤️ 1 | 🌽 50 | 0 |
|  | Scout | 3 | ⚔️ 10 | ❤️ 100 | 🌽 30 | 0 |
|  | Warrior | 2 | ⚔️ 20 | ❤️ 100 | 🌽 40 | 🪙 1 |
|  | Swordsman | 2 | ⚔️ 35 | ❤️ 100 | 🌽 90 | 🪙 2 |

#### Buildings
| Image | Building | Production | Cost | Maintenance | Construction time
|---|---|---|---|---|---|
|  | Barracks | Allows the recruitment of soldiers | 🪵 30 | 🪙 2 | 5 turns
|  | Wall | Adds ❤️ 50 to the city | 🪵 150 | 🪙 1 | 9 turns
|  | Market | 🪙 3 per turn | 🪵 120 | 0 | 7 turns
|  | Farm | 🌽 3 per turn | 1 worker | 0 | 0
|  | Lumber Mill | 🪵 3 per turn | 1 worker | 0 | 0

#### Fights with barbarians
- The likelihood of encountering barbarians increases with each turn.
- Players can send soldiers to fight barbarians.
- If successful, players earn extra $GOLD and $FOOD.
- If unsuccessful, soldiers may be lost.

#### End of the game
- Player wins when all enemies are defeated.
- Player can restart the game with a new generated map at any time.

## Future expansions
- Multiplayer PvP battles.
- More building and unit types.
- Technology tree (research)
- Building and unit upgrades.
- Epochs
- Alliance and trading mechanisms.
- And more!
