#!/bin/bash
# Post-build UF2 generation script for both Pico and XIAO RP2040

# Function to generate UF2 for a specific target
generate_uf2() {
    local target_name=$1
    local elf_file="target/thumbv6m-none-eabi/release/${target_name}"
    local uf2_file="target/${target_name}.uf2"
    
    echo "🔄 Converting ${target_name} ELF to UF2..."
    if [ -f "$elf_file" ]; then
        if picotool uf2 convert "$elf_file" -t elf "$uf2_file"; then
            echo "✅ UF2 file generated: $uf2_file"
        else
            echo "❌ Failed to generate UF2 file for ${target_name}"
            return 1
        fi
    else
        echo "❌ ELF file not found: $elf_file"
        return 1
    fi
}

# Wait a moment for the build to complete
sleep 1

# Generate UF2 for both targets
echo "🚀 Generating UF2 files for both targets..."

# Check if we're building for a specific target
if [ "$1" = "pico" ]; then
    generate_uf2 "neontap_pico"
elif [ "$1" = "xiao" ]; then
    generate_uf2 "neontap_xiao"
else
    # Generate both if no specific target specified
    generate_uf2 "neontap_pico"
    generate_uf2 "neontap_xiao"
fi

echo "🎉 UF2 generation complete!"
