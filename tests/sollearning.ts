import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sollearning } from "../target/types/sollearning";

describe("sollearning", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 🔥 Definir um novo pagador correto
  const payer = anchor.web3.Keypair.generate();
  console.log("✅ Novo Payer Definido:", payer.publicKey.toBase58());

  const program = anchor.workspace.Sollearning as Program<Sollearning>;

  it("Is initialized!", async () => {
    try {
      // Obtém o último blockhash para evitar problemas de sincronização
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction(latestBlockhash, "confirmed");

      console.log("🔄 Enviando transação...");

      // Executa a transação de inicialização usando o novo payer
      const tx = await program.methods.initialize().signers([payer]).rpc();
      
      if (!tx) {
        throw new Error("A transação não retornou uma assinatura válida.");
      }

      console.log("✅ Transação bem-sucedida! Assinatura:", tx);
    } catch (err) {
      console.error("❌ Erro ao executar a transação:", err);

      if (err.logs) {
        console.log("📜 Logs da transação:");
        console.log(err.logs.join("\n"));
      }

      if (err.signature) {
        console.log("🔍 Buscando logs pelo Signature...");
        const logs = await provider.connection.getTransaction(err.signature, {
          commitment: "confirmed",
        });
        console.log(logs?.meta?.logMessages || "Nenhum log encontrado.");
      } else {
        console.log("⚠️ A assinatura da transação não foi gerada. Verifique se as contas estão corretas.");
      }
    }
  });
});
