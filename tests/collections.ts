import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Program } from "@project-serum/anchor";
import { Collections } from "../target/types/collections";

describe("collections", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const anyAnchor: any = anchor;
  const program = anyAnchor.workspace.Collections as Program<Collections>;

  let collection: Pda;

  interface Pda {
    address: PublicKey;
    bump: number;
  }

  it("config", async () => {
    let [a, b] = await findCollectionAddress("test");
    collection = {
      address: a,
      bump: b,
    };
  });

  it("create collection", async () => {
    let t: String = "test";
    const tx = await program.rpc.createCollection(collection.bump, t, {
      accounts: {
        creator: provider.wallet.publicKey,
        collection: collection.address,
        systemProgram: SystemProgram.programId,
      },
    });
    console.log("Your transaction signature", tx);
  });
});

const COLLECTIONS_PROGRAM_ID = new PublicKey(
  "G3am3SCcStwk8gCXfVEADAnjgBRDRHv6ap7QXjKjQstq"
);
export const findCollectionAddress = async (name: String) => {
  return PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode("collection"),
      anchor.utils.bytes.utf8.encode(name.toLowerCase()),
    ],
    COLLECTIONS_PROGRAM_ID
  );
};
