import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap-icons/font/bootstrap-icons.css';

import App from './App.svelte';


const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;
