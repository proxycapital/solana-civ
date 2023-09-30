import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solciv } from "../target/types/solciv";
import { expect } from "chai";

describe("solciv", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solciv as Program<Solciv>;

  const [gameKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("GAME"), provider.publicKey.toBuffer()],
    program.programId
  );

  const [playerKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("PLAYER"), gameKey.toBuffer(), provider.publicKey.toBuffer()],
    program.programId
  );

  const [npcKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("NPC"), gameKey.toBuffer()],
    program.programId
  );

  async function addToProductionQueue(cityId, item) {
    const accounts = {
      player: provider.publicKey,
      playerAccount: playerKey,
    };
    await program.methods.addToProductionQueue(cityId, item).accounts(accounts).rpc();
  }
  
  function checkProductionQueue(account, cityId, expectedQueue) {
    const city = account.cities[cityId];
    expect(city.productionQueue.length).equal(expectedQueue.length);
    expectedQueue.forEach((item, index) => {
      expect(city.productionQueue[index]).deep.equal(item);
    });
  }

  it("Initialize game", async () => {
    // generate random 20x20 map with tile types from 1 to 9
    const randomMap = Array.from({ length: 400 }, () => Math.floor(Math.random() * 9) + 1);

    // this is needed for the future test of upgrading tiles
    // Builder is initialized at (3, 2) coordinates
    // "2" value is the forest tile type that can be upgraded to TimberCamp
    randomMap[3 + 2 * 20] = 2;

    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    const tx = await program.methods.initializeGame(randomMap).accounts(accounts).rpc();
    const account = await program.account.game.fetch(gameKey);

    expect(account.player.toBase58()).equal(provider.publicKey.toBase58());
    expect(account.map).deep.equal(randomMap);
  });

  it("Initialize player with units and balances", async () => {
    const accounts = {
      game: gameKey,
      playerAccount: playerKey,
      player: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    const tx = await program.methods.initializePlayer().accounts(accounts).rpc();
    const account = await program.account.player.fetch(playerKey);

    expect(account.units.length).equal(3);
    expect(account.nextUnitId).equal(3);
  });

  it("Initialize NPC with units and cities", async () => {
    const accounts = {
      game: gameKey,
      npcAccount: npcKey,
      player: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    const tx = await program.methods.initializeNpc().accounts(accounts).rpc();
    const account = await program.account.npc.fetch(npcKey);

    expect(account.units.length).equal(3);
    expect(account.cities.length).equal(0);
    expect(account.nextUnitId).equal(3);
  });

  it("Should attack barbarian", async () => {
    const accounts = {
      game: gameKey,
      playerAccount: playerKey,
      npcAccount: npcKey,
      player: provider.publicKey,
    };
    const unitId = 2;
    const barbarianId = 2;
    const tx = await program.methods.attackUnit(unitId, barbarianId).accounts(accounts).rpc();
    const account = await program.account.npc.fetch(npcKey);
    const playerData = await program.account.player.fetch(playerKey);
    expect(account.units[barbarianId].health).lessThan(100);
    expect(playerData.units[unitId].health).lessThan(100);
  });

  it("Move unit", async () => {
    const accounts = {
      playerAccount: playerKey,
      player: provider.publicKey,
    };
    const unitId = 0;
    const x = 1;
    const y = 1;
    await program.methods.moveUnit(unitId, x, y).accounts(accounts).rpc();
    const account = await program.account.player.fetch(playerKey);
    expect(account.units[unitId].x).equal(x);
    expect(account.units[unitId].y).equal(y);
  });

  it("Should fail to move unit", async () => {
    const accounts = {
      playerAccount: playerKey,
      player: provider.publicKey,
    };
    // Cannot move out of 20x20 map bounds
    try {
      await program.methods.moveUnit(0, 1, 100).accounts(accounts).rpc();
    } catch (e) {
      const { message } = e;
      expect(message).include("OutOfMapBounds");
    }
    // Cannot move farther than moving_range
    try {
      await program.methods.moveUnit(0, 1, 10).accounts(accounts).rpc();
    } catch (e) {
      const { message } = e;
      expect(message).include("CannotMove");
    }
  });

  it("Found the city", async () => {
    // get player account and find unit of type "settler"
    const playerAccount = await program.account.player.fetch(playerKey);
    const unitId = playerAccount.units.findIndex((unit) => Object.keys(unit.unitType)[0] === "settler");
    const unit = playerAccount.units[unitId];

    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      playerAccount: playerKey,
      // city: cityKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    await program.methods.foundCity(unit.x, unit.y, unitId).accounts(accounts).rpc();

    const player = await program.account.player.fetch(playerKey);
    expect(player.nextCityId).equal(1);
    // settler unit should be removed
    expect(player.units.length).equal(2);

    const account = await program.account.player.fetch(playerKey);
    const city = account.cities[0];
    expect(account.player.toBase58()).equal(provider.publicKey.toBase58());
    expect(city.x).equal(unit.x);
    expect(city.y).equal(unit.y);
    expect(city.cityId).equal(0);
  });

  it("Should add building to production queue", async () => {
    const cityId = 0;
    const productionItem = { building: { '0': { forge: {} } } };
    await addToProductionQueue(cityId, productionItem);
  
    const account = await program.account.player.fetch(playerKey);
    checkProductionQueue(account, cityId, [productionItem]);
  });
  
  it("Should not duplicate the building in production queue", async () => {
    const cityId = 0;
    const productionItem = { building: { '0': { forge: {} } } };
    try {
      await addToProductionQueue(cityId, productionItem);
    } catch (e) {
      const { message } = e;
      expect(message).include("AlreadyQueued");
    }
  });
  
  it("Should add 4 more items to production queue", async () => {
    const cityId = 0;
    const items = [
      { unit: { '0': { warrior: {} } } },
      { unit: { '0': { warrior: {} } } },
      { unit: { '0': { builder: {} } } },
      { building: { '0': { granary: {} } } },
    ];
  
    for (const item of items) {
      await addToProductionQueue(cityId, item);
    }
  
    const account = await program.account.player.fetch(playerKey);
    const expectedQueue = [
      { building: { '0': { forge: {} } } },
      ...items,
    ];
    checkProductionQueue(account, cityId, expectedQueue);
  });

  it("Should not add 6th item to the production queue", async () => {
    const cityId = 0;
    const productionItem = { unit: { '0': { warrior: {} } } };
    try {
      await addToProductionQueue(cityId, productionItem);
    } catch (e) {
      const { message } = e;
      expect(message).include("QueueFull");
    }
  });

  it("Should not upgrade land tile using Warrior", async () => {
    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      playerAccount: playerKey,
    };
    const x = 3;
    const y = 2;
    const unit_id = 2; // warrior created in initializePlayer
    try {
      const tx = await program.methods.upgradeTile(x, y, unit_id).accounts(accounts).rpc();
      console.log("Transaction signature", tx);
    } catch (e) {
      const { message } = e;
      expect(message).include("InvalidUnitType");
    }
  });

  it("Should not upgrade land tile if the coords do not match current unit position", async () => {
    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      playerAccount: playerKey,
    };
    const x = 3;
    const y = 3;
    const unit_id = 1; // warrior created in initializePlayer
    try {
      const tx = await program.methods.upgradeTile(x, y, unit_id).accounts(accounts).rpc();
      console.log("Transaction signature", tx);
    } catch (e) {
      const { message } = e;
      expect(message).include("UnitWrongPosition");
    }
  });

  it("Upgrade land tile", async () => {
    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      playerAccount: playerKey,
    };
    const x = 3;
    const y = 2;
    const unit_id = 1; // builder created in initializePlayer
    const tx = await program.methods.upgradeTile(x, y, unit_id).accounts(accounts).rpc();

    const account = await program.account.player.fetch(playerKey);
    expect(account.tiles).deep.equal([{ tileType: { timberCamp: {} }, x, y }]);
  });

  it("Should not upgrade land tile with a Builder that was already consumed", async () => {
    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      playerAccount: playerKey,
    };
    const x = 3;
    const y = 2;
    const unit_id = 1; // builder created in initializePlayer
    try {
      await program.methods.upgradeTile(x, y, unit_id).accounts(accounts).rpc();
    } catch (e) {
      const { message } = e;
      expect(message).include("UnitNotFound.");
    }

    const account = await program.account.player.fetch(playerKey);
  });

  it("End 1st turn", async () => {
    const prevPlayerAccount = await program.account.player.fetch(playerKey);
    const accounts = {
      game: gameKey,
      playerAccount: playerKey,
      player: provider.publicKey,
      npcAccount: npcKey,
    };
    await program.methods.endTurn().accounts(accounts).rpc();
    const account = await program.account.game.fetch(gameKey);
    expect(account.turn).equal(2);
    const playerAccount = await program.account.player.fetch(playerKey);
    expect(playerAccount.resources.gold).greaterThan(prevPlayerAccount.resources.gold);
    expect(playerAccount.resources.wood).eq(2); // we have upgraded forest tile to TimberCamp
    const npcAccount = await program.account.npc.fetch(npcKey);
  });

  it("Should close game", async () => {
    const accounts = {
      game: gameKey,
      npcAccount: npcKey,
      playerAccount: playerKey,
      player: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    const prevBalance = await provider.connection.getBalance(provider.publicKey);
    await program.methods.closeGame().accounts(accounts).rpc();
    const balance = await provider.connection.getBalance(provider.publicKey);
    // verify that rent was returned
    expect(balance).greaterThan(prevBalance);
    // verify that all accounts were closed
    try {
      await program.account.game.fetch(gameKey);
    } catch (e) {
      const { message } = e;
      expect(message).include("Account does not exist or has no data");
    }
    try {
      await program.account.player.fetch(playerKey);
    } catch (e) {
      const { message } = e;
      expect(message).include("Account does not exist or has no data");
    }
    try {
      await program.account.npc.fetch(npcKey);
    } catch (e) {
      const { message } = e;
      expect(message).include("Account does not exist or has no data");
    }
  });
});
