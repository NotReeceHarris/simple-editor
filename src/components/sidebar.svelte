<script lang="ts">

    import { onDestroy, onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { readDir, readTextFile } from '@tauri-apps/plugin-fs';
    import { fileIcons } from '../icons/fileIcons';
    import { folderIcons } from '../icons/folderIcons';

    let fileTree: HTMLElement;

    onMount(async () => {
        
        const path = 'C:\\Users\\reece\\OneDrive\\Documents\\Projects\\simple-editor';
        const entries = await readDir(path);
        
        for (const entry of entries) {
            const div = document.createElement('div');
            const span = document.createElement('span');

            span.textContent = entry.name;
            div.classList.add('flex', 'place-items-center', 'gap-1.5', 'cursor-pointer', 'hover:bg-white/[.1]', 'px-2');

            if (entry.isDirectory) {

                if (folderIcons[0].icons) {

                    for (const icon of folderIcons[0].icons) {
                        for (const name of icon.folderNames) {
                            console.log(name);
                            if (entry.name.replace('.', '').toLowerCase() === name) {
                                div.innerHTML = `
                                <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619z" clip-rule="evenodd"/></svg>
                                <img class="w-3.5 h-3.5" src="/icons/${icon.name}.svg">
                                `;
                                break;
                            }
                        }
                    }

                }

                if (div.innerHTML == '') {
                    div.innerHTML = `
                        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619z" clip-rule="evenodd"/></svg>
                        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M14.5 3H7.71l-.85-.85L6.51 2h-5l-.5.5v11l.5.5h13l.5-.5v-10zm-.51 8.49V13h-12V7h4.49l.35-.15l.86-.86H14v1.5zm0-6.49h-6.5l-.35.15l-.86.86H2v-3h4.29l.85.85l.36.15H14z"/></svg>
                    `;
                }

            } else {

                for (const icon of fileIcons.icons) {
                    if (icon.fileNames) {
                        for (const name of icon.fileNames) {
                           if (entry.name.toLowerCase() === name) {
                               div.innerHTML = `<img class="w-3.5 h-3.5" src="/icons/${icon.name}.svg">`;
                               break;
                           }
                        }
                    }

                    if (icon.fileExtensions) {
                        for (const ext of icon.fileExtensions) {
                            if (entry.name.split('.').includes(ext)) {
                                div.innerHTML = `<img class="w-3.5 h-3.5" src="/icons/${icon.name}.svg">`;
                                break;
                            }
                        }
                    }
                }

                if (div.innerHTML == '') {
                    div.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="m13.71 4.29l-3-3L10 1H4L3 2v12l1 1h9l1-1V5zM13 14H4V2h5v4h4zm-3-9V2l3 3z" clip-rule="evenodd"/></svg>`;
                }

                div.classList.add('pl-[27px]')

                div.addEventListener('click', async () => {
                    const file = await readTextFile(path + '\\' + entry.name);
                    console.log(file);
                });
            }

            div.appendChild(span);

            fileTree.appendChild(div);
        }
    });

</script>

<div class="w-48 min-w-48 max-w-48 h-full border-zinc-700 border-r z-10 flex flex-col">

    <div class="border-zinc-700 border-b h-9 max-h-9 min-h-9 flex place-items-center text-white justify-evenly">
        
        <button>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M14.5 3H7.71l-.85-.85L6.51 2h-5l-.5.5v11l.5.5h13l.5-.5v-10zm-.51 8.49V13h-12V7h4.49l.35-.15l.86-.86H14v1.5zm0-6.49h-6.5l-.35.15l-.86.86H2v-3h4.29l.85.85l.36.15H14z"/></svg>
        </button>

        <button>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M13.273 7.73a2.51 2.51 0 0 0-3.159-.31a2.5 2.5 0 0 0-.921 1.12a2.2 2.2 0 0 0-.13.44a4.52 4.52 0 0 1-4-4a2.2 2.2 0 0 0 .44-.13a2.5 2.5 0 0 0 1.54-2.31a2.45 2.45 0 0 0-.19-1A2.48 2.48 0 0 0 5.503.19a2.45 2.45 0 0 0-1-.19a2.5 2.5 0 0 0-2.31 1.54a2.52 2.52 0 0 0 .54 2.73c.35.343.79.579 1.27.68v5.1a2.4 2.4 0 0 0-.89.37a2.5 2.5 0 1 0 3.47 3.468a2.5 2.5 0 0 0 .42-1.387a2.45 2.45 0 0 0-.19-1a2.48 2.48 0 0 0-1.81-1.49v-2.4a5.5 5.5 0 0 0 2 1.73a5.7 5.7 0 0 0 2.09.6a2.5 2.5 0 0 0 4.95-.49a2.5 2.5 0 0 0-.77-1.72zm-8.2 3.38c.276.117.512.312.68.56a1.5 1.5 0 0 1-2.08 2.08a1.55 1.55 0 0 1-.56-.68a1.5 1.5 0 0 1-.08-.86a1.49 1.49 0 0 1 1.18-1.18a1.5 1.5 0 0 1 .86.08M4.503 4a1.5 1.5 0 0 1-1.39-.93a1.5 1.5 0 0 1-.08-.86a1.49 1.49 0 0 1 1.18-1.18a1.5 1.5 0 0 1 .86.08A1.5 1.5 0 0 1 4.503 4m8.06 6.56a1.5 1.5 0 0 1-2.45-.49a1.5 1.5 0 0 1-.08-.86a1.49 1.49 0 0 1 1.18-1.18a1.5 1.5 0 0 1 .86.08a1.5 1.5 0 0 1 .49 2.45"/></svg>
        </button>

        <button>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 36 36"><path fill="currentColor" d="M29.81 16H29V8.83a2 2 0 0 0-2-2h-6A5.14 5.14 0 0 0 16.51 2A5 5 0 0 0 11 6.83H4a2 2 0 0 0-2 2V17h2.81A3.13 3.13 0 0 1 8 19.69A3 3 0 0 1 7.22 22A3 3 0 0 1 5 23H2v8.83a2 2 0 0 0 2 2h23a2 2 0 0 0 2-2V26h1a5 5 0 0 0 5-5.51A5.15 5.15 0 0 0 29.81 16m2.41 7A3 3 0 0 1 30 24h-3v7.83H4V25h1a5 5 0 0 0 5-5.51A5.15 5.15 0 0 0 4.81 15H4V8.83h9V7a3 3 0 0 1 1-2.22A3 3 0 0 1 16.31 4A3.13 3.13 0 0 1 19 7.19v1.64h8V18h2.81A3.13 3.13 0 0 1 33 20.69a3 3 0 0 1-.78 2.31" class="clr-i-outline clr-i-outline-path-1"/><path fill="none" d="M0 0h36v36H0z"/></svg>
        </button>

        <button>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M3.78 2L3 2.41v12l.78.42l9-6V8zM4 13.48V3.35l7.6 5.07z"/></svg>
        </button>

    </div>

    <div bind:this={fileTree} class="flex flex-col pt-2 gap-0.5 h-full text-white text-xs overflow-y-auto" />

</div>