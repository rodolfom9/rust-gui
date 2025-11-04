@echo off
REM Script para converter SVG para ICO se necessário
REM Por enquanto, apenas garante que o ícone existe
if exist "icon.svg" (
    echo Icon file found: icon.svg
) else (
    echo Error: icon.svg not found
)
