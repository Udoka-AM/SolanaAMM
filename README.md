# Anchor AMM - Q4 2024

## Overview

This project implements an Automated Market Maker (AMM) on the Solana blockchain using the Anchor framework. It enables:

- Creation and management of liquidity pools for token pairs
- Token swaps with minimal slippage and slippage protection
- Secure deposit and withdrawal of liquidity

## Architecture

The system is built around a single state account:

### Config Account

```rust
#[account]
pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}
```

This state account stores:
- `seed`: Unique identifier for different pool configurations
- `authority`: Optional admin key that can lock the pool
- `mint_x` & `mint_y`: Token pair in the pool
- `fee`: Swap fee in basis points
- `locked`: Pool lock status
- `config_bump`: Bump for Config account PDA
- `lp_bump`: Bump for Liquidity Provider PDA

## Core Functionality

### Pool Initialization

The `Initialize` context creates a new liquidity pool:

```rust
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'info, Mint>,
    // Additional accounts...
}
```

Implementation:
```rust
impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });

        Ok(())
    }
}
```

### Deposit

Users deposit tokens to the liquidity pool and receive LP tokens:

```rust
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    // Additional accounts...
}
```

Key implementation:
```rust
impl<'info> Deposit<'info> {
    pub fn deposit (
        &mut self,
        amount: u64, // Amount of LP tokens user wants to "claim"
        max_x: u64,  // Maximum token X user will deposit
        max_y: u64,  // Maximum token Y user will deposit
    ) -> Result<()> {
        // Check pool not locked
        // Calculate proportional deposit amounts
        // Handle deposits and LP token minting
    }
}
```

### Withdraw

Users can burn LP tokens to withdraw liquidity:

```rust
pub fn withdraw(
    &mut self,
    amount: u64, // LP tokens to burn
    min_x: u64,  // Minimum token X to receive
    min_y: u64,  // Minimum token Y to receive
) -> Result<()> {
    // Validate withdrawal conditions
    // Calculate withdrawal amounts
    // Check slippage protection
    // Transfer tokens and burn LP tokens
}
```

### Swap

Users can swap between pool tokens:

```rust
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    // Additional accounts...
}
```

Implementation:
```rust
pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
    // Verify pool not locked
    // Initialize constant product curve
    // Calculate swap amounts
    // Perform token transfers
}
```

## Security Features

- PDA-based ownership for vaults
- Slippage protection for all operations
- Optional authority for pool locking
- Constant product formula for price determination
