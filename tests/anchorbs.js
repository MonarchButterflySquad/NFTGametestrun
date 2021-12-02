
const anchor = require('@project-serum/anchor');
const { SystemProgram,PublicKey,Keypair } = anchor.web3;

const metakey = new PublicKey("Gi2KRtkh7VW3ZzbhocdC852R3F6y71N7fk4HyvJBR12M");
const metadataProgramIdKey = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
				
const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
   
  const program = anchor.workspace.Myepicproject;

 

let [address, nonce] = await PublicKey.findProgramAddress(
    [
        Buffer.from('signtwo')

    ],
    program.programId
);

console.log(program.programId.toString());
console.log(address.toString());




   let tx = await program.rpc.evolve(nonce,{
   accounts: {
        tokenMetadataProgram: metadataProgramIdKey,
        metaData:metakey,
   	signerAccount: address, 
   	user:provider.wallet.publicKey, 
   	systemProgram: SystemProgram.programId,
   },});
  console.log("📝 Your transaction signature", tx);



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


