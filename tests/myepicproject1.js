const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;


const main = async() => {
  console.log("Starting test...")
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.Myepicproject1;

  const baseAccount = anchor.web3.Keypair.generate();


  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],

  });

  console.log("Your transaction string: ", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log("Gif Count: ", account.totalGifs.toString())

  await program.rpc.addGif("https://media.giphy.com/media/ejJclNX60XyEo555wW/giphy.gif", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
    },
  });


  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif count", account.totalGifs.toString())
  console.log("Gif List", account.gifList)
}

const runMain = async () => {
  //run main is a functional comp that gets your main function to run async using try catch
  try{
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();