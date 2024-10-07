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
			'javascript'
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
          breadCrumb = breadCrumb + crumb.word + ' > '
        }

      });

      if (breadcrumbElm) {
        breadcrumbElm.textContent = breadCrumb;
      }

      window.dispatchEvent(new CustomEvent("editor-cursor-move", {
        detail: e.position,
      }));
    });

    resize = () => {
      editor?.layout({
        width: containerElm.clientWidth - 1,
        height: containerElm.clientHeight,
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

<style>
  :global(.monaco-editor .overlayWidgets) {
    position: fixed !important;
    top: 46px !important;
    left: 50vw !important;
    transform: translateX(-50%) !important;
    z-index: 100 !important;
  }
</style>

<div class="w-full h-full flex flex-col border-zinc-700 border-r">
  <div class="h-9 w-full bg-window border-zinc-700 border-b"></div>
  <div bind:this={breadcrumbElm} class="px-3 overflow-hidden h-9 w-full bg-editor border-zinc-700 border-b flex place-items-center text-white text-xs text-ellipsis"></div>
  <div class="w-full h-full relative bg-editor" bind:this={containerElm}>
    <div class="absolute top-0 left-0" bind:this={editorElm} />
  </div>
</div>