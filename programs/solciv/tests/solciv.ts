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
  console.log("Game account address", gameKey.toString());

  const [playerKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("PLAYER"), gameKey.toBuffer(), provider.publicKey.toBuffer()],
    program.programId
  );
  console.log("Player account address", playerKey.toString());

  it("Initialize game", async () => {
    // generate random 20x20 map with tile types from 1 to 9
    const randomMap = Array.from({length: 400}, () => Math.floor(Math.random() * 9) + 1);

    const accounts = {
      game: gameKey,
      player: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    const tx = await program.methods.initializeGame(randomMap).accounts(accounts).rpc();
    console.log("Transaction signature", tx);

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
    console.log("Transaction signature", tx);

    const account = await program.account.player.fetch(playerKey);

    expect(account.units.length).equal(3);
    expect(account.nextUnitId).equal(3);
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
    const unitId = playerAccount.units.findIndex(unit => Object.keys(unit.unitType)[0] === "settler");
    const unit = playerAccount.units[unitId];

    // const buff = Buffer.allocUnsafe(4);
    // buff.writeUInt32LE(playerAccount.nextCityId, 0);
    // const [cityKey] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("CITY"), gameKey.toBuffer(), provider.publicKey.toBuffer(), buff],
    //   program.programId
    // );
    // console.log("City account address", cityKey.toString());
    
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
    console.log(city);
    expect(account.player.toBase58()).equal(provider.publicKey.toBase58());
    expect(city.x).equal(unit.x);
    expect(city.y).equal(unit.y);
    expect(city.cityId).equal(0);
  });

  it("End 1st turn", async () => {
    const prevPlayerAccount = await program.account.player.fetch(playerKey);
    const accounts = {
      game: gameKey,
      playerAccount: playerKey,
      player: provider.publicKey,
    };
    await program.methods.endTurn().accounts(accounts).rpc();
    const account = await program.account.game.fetch(gameKey);
    expect(account.turn).equal(2);
    const playerAccount = await program.account.player.fetch(playerKey);
    expect(playerAccount.resources.gold).greaterThan(prevPlayerAccount.resources.gold);
  });
});
