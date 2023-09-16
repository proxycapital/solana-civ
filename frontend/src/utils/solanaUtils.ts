import {
  PublicKey,
  Connection,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  TransactionInstruction,
  Keypair,
  clusterApiUrl,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { Buffer } from "buffer";
import bs58 from "bs58";
import { weightedRandomTile } from "../components/Terrain";

const { REACT_APP_RPC: RPC, REACT_APP_PROGRAM_ID: PROGRAM_ADDRESS } = process.env;

if (!PROGRAM_ADDRESS) {
  throw new Error("REACT_APP_PROGRAM_ID is undefined. Hint: `cp .env.sample .env`");
}
// if (!localStorage.getItem("solanaWalletSecretKey")) {
//     throw new Error("Cannot find wallet keypair in local storage");
// }
const PROGRAM_ID = new PublicKey(PROGRAM_ADDRESS);

const connection = new Connection(RPC || clusterApiUrl("devnet"), "confirmed");

class MapPayload {
  myMap: number[] = [];
}

const MapSchema = {
  struct: {
    myMap: { array: { type: "u8", len: 400 } },
  },
};

const BalancesSchema = {
  struct: {
    is_initialized: "bool",
    gold: "u64",
    food: "u64",
    lumber: "u64",
  },
};

const getAccountInfo = async (discriminator: string, pubkey: PublicKey) => {
  const seeds = [Buffer.from(discriminator), pubkey.toBytes()];
  const [pdaPublicKey, _bump] = await PublicKey.findProgramAddress(seeds, PROGRAM_ID);
  console.log(`${discriminator} PDA:`, pdaPublicKey.toBase58());

  const accountInfo = await connection.getAccountInfo(pdaPublicKey);
  return { accountInfo, pdaPublicKey };
};

export const getMap = async () => {
  const wallet = Keypair.fromSecretKey(bs58.decode(localStorage.getItem("solanaWalletSecretKey")!));
  const { accountInfo, pdaPublicKey } = await getAccountInfo("game", wallet.publicKey);

  if (accountInfo === null) {
    console.error("Error: cannot find the game account");
    // @todo: add a button to initialize the game ?
    return;
  }
  const data = Buffer.from(accountInfo.data);
  const map = borsh.deserialize(MapSchema, data) as MapPayload;
  console.log(map);
  if (!map || !map.myMap) {
    console.error("Error: cannot deserialize the game account data");
    return;
  }
  return map.myMap;
};

export const fetchBalances = async () => {
  const publicKeyString = localStorage.getItem("solanaWalletPublicKey");
  if (!publicKeyString) return null;

  const publicKey = new PublicKey(publicKeyString);
  const balances = {
    gold: 0,
    food: 0,
    lumber: 0,
    sol: 0,
  };
  try {
    const [balance, info] = await Promise.all([
      connection.getBalance(publicKey),
      getAccountInfo("balances", publicKey),
    ]);
    const accountInfo = info.accountInfo;
    balances.sol = balance ? Number(balance) / 1e9 : 0;
    if (accountInfo !== null && typeof accountInfo !== "number") {
      const resources = borsh.deserialize(BalancesSchema, Buffer.from(accountInfo.data));
      if (resources) {
        console.log(resources);
        balances.gold = Number((resources as any).gold);
        balances.food = Number((resources as any).food);
        balances.lumber = Number((resources as any).lumber);
      }
    }
  } catch (error) {
    console.error("Failed to fetch balance", error);
  }
  return balances;
};

export const initializeGame = async () => {
  const wallet = Keypair.fromSecretKey(bs58.decode(localStorage.getItem("solanaWalletSecretKey")!));
  const transaction = new Transaction();
  const gamePayload = new MapPayload();
  gamePayload.myMap = Array.from({ length: 400 }, () => weightedRandomTile());

  const { accountInfo: gameInfo, pdaPublicKey: gamePdaKey } = await getAccountInfo("game", wallet.publicKey);

  if (gameInfo === null) {
    const instructionData = [0, ...borsh.serialize(MapSchema, gamePayload)];
    const initGameIx = new TransactionInstruction({
      keys: [
        { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: gamePdaKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from(instructionData),
    });
    transaction.add(initGameIx);
  }

  const { accountInfo, pdaPublicKey: playerPdaKey } = await getAccountInfo("balances", wallet.publicKey);
  if (accountInfo === null) {
    const initPlayerIx = new TransactionInstruction({
      keys: [
        { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: playerPdaKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([1]),
    });
    transaction.add(initPlayerIx);
  }
  if (transaction.instructions.length > 0) {
    const signedTransaction = await sendAndConfirmTransaction(connection, transaction, [wallet], {
      preflightCommitment: "single",
      skipPreflight: false,
    });

    console.log("Transaction confirmed:", signedTransaction);
  }
};
