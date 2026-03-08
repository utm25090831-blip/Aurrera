use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("c6VSGt8duN5FUUa8EXGSYbHLGX75wKrgFpC5cX14ecE");

#[program]
pub mod aurrera_inventario {
    use super::*;

    // --- 1. CREATE (Crear/Registrar Producto) ---
    pub fn registrar_producto(
        ctx: Context<RegistrarProducto>, 
        nombre: String, 
        categoria: String,
        stock: u64,
    ) -> Result<()> {
        let producto = &mut ctx.accounts.cuenta_producto;
        producto.encargado = *ctx.accounts.encargado.key;
        producto.nombre = nombre;
        producto.categoria = categoria;
        producto.stock = stock;
        producto.estado = "En Bodega".to_string(); // Estado inicial

        msg!("CREATE: Producto {} registrado por el encargado.", producto.nombre);
        Ok(())
    }

    // --- 2. UPDATE (Actualizar Stock y Estado) ---
    pub fn actualizar_producto(
        ctx: Context<ActualizarProducto>,
        _nombre: String, // Se usa en los seeds del Context
        nuevo_stock: u64,
        nuevo_estado: String,
    ) -> Result<()> {
        let producto = &mut ctx.accounts.cuenta_producto;
        
        // Actualizamos los datos (Lógica del Update)
        producto.stock = nuevo_stock;
        producto.estado = nuevo_estado;

        msg!("UPDATE: Producto {} actualizado. Nuevo stock: {}", producto.nombre, nuevo_stock);
        Ok(())
    }

    // --- 3. DELETE (Dar de baja / Eliminar Producto) ---
    // En Solana, borrar significa cerrar la cuenta y recuperar el dinero (rent)
    pub fn eliminar_producto(_ctx: Context<EliminarProducto>, nombre: String) -> Result<()> {
        msg!("DELETE: Producto {} eliminado del sistema. Fondos devueltos.", nombre);
        Ok(())
    }
}

// --- ESTRUCTURA DE DATOS (El "Estado") ---
#[account]
#[derive(InitSpace)]
pub struct ProductoAccount {
    pub encargado: Pubkey,   // 32 bytes
    #[max_len(32)]
    pub nombre: String,      // Nombre del producto (ej. "Leche")
    #[max_len(32)]
    pub categoria: String,   // Categoría (ej. "Lácteos")
    pub stock: u64,          // Cantidad disponible
    #[max_len(50)]
    pub estado: String,      // Estado (ej. "Oferta", "Agotado")
}

// --- CONTEXTOS DE VALIDACIÓN ---

#[derive(Accounts)]
#[instruction(nombre: String)] 
pub struct RegistrarProducto<'info> {
    #[account(
        init, // Indica que vamos a CREAR una cuenta
        seeds = [nombre.as_bytes(), encargado.key().as_ref()], // Semillas únicas
        bump,
        payer = encargado,
        space = 8 + ProductoAccount::INIT_SPACE // Espacio necesario en la blockchain
    )]
    pub cuenta_producto: Account<'info, ProductoAccount>,
    #[account(mut)]
    pub encargado: Signer<'info>, // El usuario que paga y firma
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct ActualizarProducto<'info> {
    #[account(
        mut, // Indica que vamos a MODIFICAR la cuenta
        seeds = [nombre.as_bytes(), encargado.key().as_ref()],
        bump,
        has_one = encargado // Seguridad: solo el encargado original puede editar
    )]
    pub cuenta_producto: Account<'info, ProductoAccount>,
    pub encargado: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct EliminarProducto<'info> {
    #[account(
        mut,
        seeds = [nombre.as_bytes(), encargado.key().as_ref()],
        bump,
        close = encargado // Indica que vamos a CERRAR la cuenta (DELETE)
    )]
    pub cuenta_producto: Account<'info, ProductoAccount>,
    pub encargado: Signer<'info>,
}