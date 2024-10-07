<script lang="ts">
    import { onDestroy, onMount } from 'svelte';

    let cursorLine: HTMLElement | null;
    let cursorColumn: HTMLElement | null;

    onMount(() => {

        window.addEventListener("editor-cursor-move", (e) => {
            const position = (e as any).detail;
            const line = position.lineNumber;
            const column = position.column;

            if (!cursorLine || !cursorColumn) return;

            cursorLine.textContent = line.toString();
            cursorColumn.textContent = column.toString();
        });

    })

</script>


<div class="h-9 px-3 text-white text-[10px] w-screen border-zinc-700 border-t flex place-items-center justify-between">

    <div class="grow flex"></div>

    <div class="grow flex justify-end">
        <div class="flex gap-2">
            <span>
                Ln <span bind:this={cursorLine}>0</span>,
            </span>
            <span>
                Col <span bind:this={cursorColumn}>0</span>
            </span>
        </div>
    </div>

</div>