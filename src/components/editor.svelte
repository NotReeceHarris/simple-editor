<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;

	let editorElm: HTMLElement;
  let containerElm: HTMLElement;
  let breadcrumbElm: HTMLElement;

  let resize: () => void;

	onMount(async () => {
		monaco = (await import('../monaco')).default;

    monaco.editor.defineTheme("default", {
      base: "vs-dark",
      inherit: true,
      rules: [
        { token: "identifier", foreground: "9CDCFE" },
        { token: "identifier.function", foreground: "DCDCAA" },
        { token: "type", foreground: "1AAFB0" },
      ],
      colors: {
        "editor.background": "#27282a",
        "scrollbar.shadow": "#27282a",
        "scrollbarSlider.background": "#353638b3",
      },
    });

		const editor = monaco.editor.create(editorElm);
		const model = monaco.editor.createModel(
			"console.log('Hello from Monaco! (the editor, not the city...)')",
			'typescript'
		);

		editor.setModel(model);
    editor.updateOptions({ 
      automaticLayout: false,
      theme: "default",
    });

    editor.onDidChangeCursorPosition((e) => {

      let breadCrumb = ''

      model.getDecorationsInRange({
        startLineNumber: 1,
        startColumn: 1,
        endLineNumber: e.position.lineNumber,
        endColumn: e.position.column,
      }).forEach((decoration) => {
        const crumb = model.getWordAtPosition(decoration.range.getStartPosition())

        if (crumb) {
          breadCrumb = breadCrumb + ' > ' + crumb.word
        }

      });

      if (breadcrumbElm) {
        breadcrumbElm.textContent = breadCrumb.slice(3);
      }

      window.dispatchEvent(new CustomEvent("editor-cursor-move", {
        detail: e.position,
      }));
    });

    resize = () => {
      editor?.layout({
        width: containerElm.clientWidth - 1,
        height: containerElm.clientHeight - 8,
      });
    };

    window.addEventListener("resize", resize);
    resize();
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
    window.removeEventListener("resize", resize);
	});
</script>

<div class="w-full h-full flex flex-col border-zinc-700 border-r">

  <!-- File list -->
  <div class="h-9 min-h-9 max-h-9 w-full bg-window border-zinc-700 border-b" />

  <!-- Breadcrumbs -->
  <div class="px-3 h-7 min-h-7 max-h-7 max-w-full w-full bg-editor border-zinc-700 border-b flex place-items-center text-white text-xs overflow-hidden">
    <span class="line-clamp-1" bind:this={breadcrumbElm} />
  </div>
  
  <!-- Monaco editor -->
  <div class="w-full h-full relative bg-editor" bind:this={containerElm}>
    <div class="absolute top-2 left-0" bind:this={editorElm} />
  </div>

  <!-- This is the cover the scrollbar line and also cover the spacing at the top of the editor -->
  <div class="absolute right-0 border-zinc-700 border-l w-4 h-[calc(100%-128px)] top-[100px]"/>
</div>