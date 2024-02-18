const anchor = require("@coral-xyz/anchor");
const { PublicKey } = require("@solana/web3.js");
const { SystemProgram } = anchor.web3;
const { createAccount } = require("@solana/spl-token");

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Clamespl;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your initialize signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalAdd.toString());


  const newUserPublicKey = new PublicKey("38J8f4VG8syxqWmRsYt8pbFf8Edeh1hoPvAuBnF4Vp6U"); 
  const nonWhitelisted = new PublicKey("6GacZCa8BsmQDcagkAK8wSF24x2EpGHkX1cuHUGEA9jj");

  // passing address :- whitelidt address
  await program.rpc.addAddress(new anchor.BN(10**9*90),newUserPublicKey, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey, 
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Address Count', account.totalAdd.toString())

  // Access gif_list on the account!
  console.log('👀 Address List', account.addList);

 // Call the check function with the caller's public key, it will check that the calle clame function is whitelisted address or not.
  const validate = await program.rpc.check({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  console.log('👀 Validate here ', validate);

  // call the create mint function 

  const mintToken = anchor.web3.Keypair.generate()

  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

  const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})

  console.log(mintToken.publicKey.toBase58())
  console.log(tokenAccount.toBase58())

  const minting = await program.methods.createToken(9,new anchor.BN(10**9*100))
        .accounts({
          mintToken:mintToken.publicKey,
          tokenAccount:tokenAccount,
          associateTokenProgram,
        })
        .signers([mintToken])
        .rpc();
        console.log("Your Minting signature", minting);

  // transfer the spl token :

        let reciever = anchor.web3.Keypair.generate()

        const signature = await provider.connection.requestAirdrop(reciever.publicKey,anchor.web3.LAMPORTS_PER_SOL)
        await provider.connection.confirmTransaction(signature)
    
        let recieverTokenAccountKeypair = anchor.web3.Keypair.generate()
        await createAccount(provider.connection,reciever,mintToken.publicKey,reciever.publicKey,recieverTokenAccountKeypair);
    
    const clame = await program.methods.transerToken(new anchor.BN(10**9*90))
      .accounts({
        mintToken:mintToken.publicKey,
        fromAccount:tokenAccount,
        toAccount:recieverTokenAccountKeypair.publicKey,
        associateTokenProgram
      })
      .signers([])
      .rpc()

      console.log("Your Clame signature", clame);
  
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();

