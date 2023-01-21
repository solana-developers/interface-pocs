import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  Keypair,
  VersionedTransaction,
  VersionedMessage,
  MessageV0,
  Message,
  TransactionMessage,
  SystemProgram,
} from "@solana/web3.js";
import { readFileSync } from "fs";
import { cwd } from "process";
import { join } from "path";

function getKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(readFileSync(path, "utf8").toString()))
  );
}

async function main() {
  console.log("Doing stuff");

  const connection = new Connection("http://localhost:8899", "confirmed");
  const kp = getKeypairFromFile("/Users/noahgundotra/.config/solana/id.json");
  await connection.requestAirdrop(kp.publicKey, 1000000000);

  const programId = getKeypairFromFile(
    join(cwd(), "..", "..", "target", "deploy", "hello_world-keypair.json")
  ).publicKey;

  connection.onLogs(programId, (logs, ctx) => {
    console.log(ctx);
    logs.logs.forEach((log) => {
      if (log.includes("Runtime")) {
        console.log(log);
        connection.removeOnLogsListener(0);
      }
    });
  });

  const ix: TransactionInstruction = {
    keys: [
      {
        pubkey: kp.publicKey,
        isWritable: true,
        isSigner: true,
      },
      {
        pubkey: Keypair.generate().publicKey,
        isWritable: true,
        isSigner: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: Buffer.from("Yikes"),
  };
  const tx = new VersionedTransaction(
    new TransactionMessage({
      instructions: [ix],
      payerKey: kp.publicKey,
      recentBlockhash: (await connection.getRecentBlockhash()).blockhash,
    }).compileToV0Message()
  );
  tx.sign([kp]);
  const txId = await connection.sendTransaction(tx, { skipPreflight: true });
  console.log("txId", txId);
}

main();
