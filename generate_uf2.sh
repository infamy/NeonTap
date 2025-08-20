#!/bin/bash
# Post-build UF2 generation script

ELF_FILE="target/thumbv6m-none-eabi/release/neontap"
UF2_FILE="target/neontap.uf2"

# Wait a moment for the build to complete
sleep 1

# Check if ELF file exists
if [ -f "$ELF_FILE" ]; then
    echo "🔄 Converting ELF to UF2..."
    if picotool uf2 convert "$ELF_FILE" -t elf "$UF2_FILE"; then
        echo "✅ UF2 file generated: $UF2_FILE"
    else
        echo "❌ Failed to generate UF2 file"
        exit 1
    fi
else
    echo "❌ ELF file not found: $ELF_FILE"
    exit 1
fi
