use anchor_lang::prelude::*;

// ID del programa en mi Playground
declare_id!("ALENsAXjrZFgmhdW92gqWPvuBfdxFYcPp4KMDRuYFWBE");

#[program]
pub mod task_manager {
    use super::*;

    // --- CREATE: Agregar una nueva tarea ---
    // Registro la tarea vinculándola a una materia y asignando su fecha de entrega
    pub fn crear_tarea(
        ctx: Context<CrearTarea>, 
        id_tarea: u64, 
        materia: String, 
        descripcion: String, 
        fecha_vencimiento: i64 
    ) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;
        
        tarea.estudiante = *ctx.accounts.estudiante.key;
        tarea.id = id_tarea;
        tarea.materia = materia;
        tarea.descripcion = descripcion;
        tarea.fecha_vencimiento = fecha_vencimiento;
        tarea.completada = false; // Por defecto la tarea está pendiente
        
        msg!("Tarea '{}' de la materia '{}' guardada", tarea.descripcion, tarea.materia);
        Ok(())
    }

    // --- UPDATE: Marcar como completada ---
    // Cambio el estado de la tarea cuando termino mi pendiente
    pub fn marcar_completada(ctx: Context<ActualizarTarea>) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;
        tarea.completada = true;
        msg!("¡Tarea completada! Un pendiente menos.");
        Ok(())
    }

    // --- UPDATE: Cambiar fecha de entrega ---
    // Por si el profesor da más tiempo o me equivoqué al anotar
    pub fn reprogramar_tarea(ctx: Context<ActualizarTarea>, nueva_fecha: i64) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;
        tarea.fecha_vencimiento = nueva_fecha;
        Ok(())
    }

    // --- DELETE: Eliminar tarea ---
    // Para limpiar mi lista y recuperar los SOL de la renta
    pub fn eliminar_tarea(_ctx: Context<EliminarTarea>) -> Result<()> {
        msg!("Tarea eliminada de la blockchain.");
        Ok(())
    }
}

// --- Validación de cuentas y PDAs ---

#[derive(Accounts)]
#[instruction(id_tarea: u64)]
pub struct CrearTarea<'info> {
    // Uso "task" + mi llave + ID para que cada tarea tenga su propio espacio
    #[account(
        init, 
        payer = estudiante, 
        space = 8 + 32 + 8 + 40 + 100 + 8 + 1, // Espacio para todos los campos
        seeds = [b"task", estudiante.key().as_ref(), id_tarea.to_le_bytes().as_ref()],
        bump
    )]
    pub tarea: Account<'info, TareaState>,
    
    #[account(mut)]
    pub estudiante: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarTarea<'info> {
    #[account(mut, has_one = estudiante)] 
    pub tarea: Account<'info, TareaState>,
    pub estudiante: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarTarea<'info> {
    #[account(mut, has_one = estudiante, close = estudiante)]
    pub tarea: Account<'info, TareaState>,
    #[account(mut)]
    pub estudiante: Signer<'info>,
}

// --- Estado de la Tarea ---

#[account]
pub struct TareaState {
    pub estudiante: Pubkey,
    pub id: u64,
    pub materia: String,
    pub descripcion: String,
    pub fecha_vencimiento: i64,
    pub completada: bool,
}
