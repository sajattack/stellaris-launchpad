//! Initialises the PLL on the LM4F120

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_store, volatile_load};
use lm4f120h5qr::registers;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(PartialEq)]
pub enum ClockSpeed {
	Speed66MHz,
	Speed16MHz
}

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************


/// Sets the clock speed to the given clock speed.
pub fn init(speed: ClockSpeed) {

    // Set up a 16MHz crystal first
    unsafe {
        let mut rcc: usize = volatile_load(registers::SYSCTL_RCC_R);
        // RCC SYSDIV field = 0x0
        rcc &= !registers::SYSCTL_RCC_SYSDIV_M;
        // XTAL field = 0x15 (=> 16MHz)
        rcc &= !registers::SYSCTL_RCC_XTAL_M;
        rcc |= registers::SYSCTL_RCC_XTAL_16MHZ;
        // Set BYPASS bit
        rcc |= registers::SYSCTL_RCC_BYPASS;
        // Set to MOSC
        rcc &= !registers::SYSCTL_RCC_OSCSRC_M;
        rcc |= registers::SYSCTL_RCC_OSCSRC_MAIN;
        // Disable PIOSC
        rcc |= registers::SYSCTL_RCC_IOSCDIS;
        // Enable MOSC (i.e. don't disable)
        rcc &= !registers::SYSCTL_RCC_MOSCDIS;
        // Write to register
        volatile_store(registers::SYSCTL_RCC_R, rcc);

        //
        // Now we're going to run at 66.67 MHz which is a ratio of 1:6 with the 400MHz PLL.
        // As the PLL is divided down by two, we need a divisor of 3.
        //
        // We could get 80MHz if we danced with RCC2 instead and got 400MHz / 5.
        //
	    if speed == ClockSpeed::Speed66MHz {

	        // Clear PLL lock status
	        // MISC = Masked Interrupt Status & Clear, not miscellaneous
	        volatile_store(registers::SYSCTL_MISC_R, registers::SYSCTL_MISC_PLLLMIS);

	        // Enable the PLL. We're OK, BYPASS is still set
	        rcc &= !registers::SYSCTL_RCC_PWRDN;
	        volatile_store(registers::SYSCTL_RCC_R, rcc);

	        // Wait for PLL to lock
	        while (volatile_load(registers::SYSCTL_RIS_R) & registers::SYSCTL_MISC_PLLLMIS) == 0 {
	            asm!("NOP");
	        }

	        // Set up a /3 divider (by putting 0x02 in the bitfield)
	        rcc &= !registers::SYSCTL_RCC_SYSDIV_M;
	        rcc |= 2 << registers::SYSCTL_RCC_SYSDIV_S;
	        // Enable use of divider
	        rcc |= registers::SYSCTL_RCC_USESYSDIV;
	        // Switch to PLL
	        rcc &= !registers::SYSCTL_RCC_BYPASS;
	        volatile_store(registers::SYSCTL_RCC_R, rcc);
	    }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************