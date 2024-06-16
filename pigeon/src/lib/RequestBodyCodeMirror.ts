import { basicSetup, EditorView } from 'codemirror';
import { EditorState } from '@codemirror/state';
import { keymap, lineNumbers } from '@codemirror/view';
import { indentWithTab } from '@codemirror/commands';
import { json, jsonParseLinter } from '@codemirror/lang-json';
import { linter, lintGutter } from '@codemirror/lint';
import type { Request } from '$lib/Models';

let myTheme = EditorView.theme({
	'&': {
		fontSize: "10pt",
		color: '#E0E0E0',
		backgroundColor: '#263238'
	},
	'&.cm-focused .cm-cursor': {
		borderLeftColor: '#FFCC80'
	},
	'.cm-content': {
		caretColor: '#FFCC80'
	},
	'&.cm-focused .cm-selectionBackground, ::selection': {
		backgroundColor: '#546E7A'
	},
	'.cm-gutters': {
		backgroundColor: '#37474F',
		color: '#B0BEC5',
		border: 'none'
	}
}, { dark: true });

export function getCodeMirror(request: Request): EditorView {
	let startState = EditorState.create({
		doc: request.body.content,
		extensions: [
			keymap.of([indentWithTab]),
			json(),
			lintGutter(),
			linter(jsonParseLinter()),
			lineNumbers(),
			myTheme
		]
	});
	return new EditorView({
		state: startState,
		value: request.body,
		extensions: [basicSetup, keymap.of([indentWithTab])],
		// @ts-ignore
		parent: document.querySelector('#body')
	});
}
