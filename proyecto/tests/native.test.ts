import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TaskManager } from "../target/types/task_manager";

describe("task-manager", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TaskManager as Program<TaskManager>;

  it("Registra una tarea de la universidad", async () => {
    const idTarea = new anchor.BN(500); // ID único para mi tarea
    const materia = "Sistemas Distribuidos";
    const descripcion = "Proyecto Final en Rust";
    const fechaVenc = new anchor.BN(Math.floor(Date.now() / 1000) + 86400 * 7); // Vence en 7 días

    // Calculo la dirección única (PDA) de la tarea
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"), 
        provider.wallet.publicKey.toBuffer(), 
        idTarea.toArrayLike(Buffer, "le", 8) 
      ],
      program.programId
    );

    // Llamo a mi programa para guardar la tarea
    await program.methods
      .crearTarea(idTarea, materia, descripcion, fechaVenc)
      .accounts({
        tarea: taskPda,
        estudiante: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verifico que se guardó bien
    const account = await program.account.tareaState.fetch(taskPda);
    console.log("✅ Tarea anotada:", account.descripcion, "para la materia:", account.materia);
  });
});