import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import fs from "fs";
import path from "path";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const idl = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../target/idl/anchor_profile.json"), "utf8")
  );
  const program = new Program(idl, provider);
  const wallet = provider.wallet.publicKey;

  const keys = await program.methods
    .createProfile("x", "x")
    .accounts({ owner: wallet })
    .pubkeys();
  const profilePda = keys.profile!;
  console.log("Кошелёк:", wallet.toBase58());
  console.log("Program ID:", program.programId.toBase58());
  console.log("Profile PDA:", profilePda.toBase58());

  const existing = await provider.connection.getAccountInfo(profilePda);
  if (!existing) {
    console.log("--- create_profile ---");
    const sig = await program.methods
      .createProfile("Zangar", "Solana developer student")
      .accounts({ owner: wallet })
      .rpc();
    console.log("Транзакция create_profile:", sig);
  } else {
    console.log("Профиль бұрыннан бар, create өткізілді");
  }

  let profile = await (program.account as any).profile.fetch(profilePda);
  console.log("Оқылды:", profile.name, "|", profile.bio);

  console.log("--- update_profile ---");
  const sig2 = await program.methods
    .updateProfile("Zangar R.", "Building on Solana")
    .accounts({ owner: wallet })
    .rpc();
  console.log("Транзакция update_profile:", sig2);

  profile = await (program.account as any).profile.fetch(profilePda);
  console.log("=== Финальное состояние профиля ===");
  console.log("owner:", profile.owner.toBase58());
  console.log("name:", profile.name);
  console.log("bio:", profile.bio);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
