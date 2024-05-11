// place files you want to import through the `$lib` alias in this folder.

import { invoke } from '@tauri-apps/api/tauri';

type GlobalTag = {
	pid: Number;
	name: string;
};

type GlobalPermissionLevel = {
	id: Number;
	level_name: string;
};

let globalTags: GlobalTag[] = [];
let globalPermissionLevels: GlobalPermissionLevel[] = [];

export async function getGlobalTags(): Promise<GlobalTag[]> {
	if (globalTags.length === 0) {
		try {
			globalTags = await invoke<GlobalTag[]>('plugin:tag|get_global_tags');
		} catch (e) {
			console.log(e);
			return Promise.reject(e);
		}
	}

	return Promise.resolve(globalTags);
}

export async function getGlobalPermissionLevels(): Promise<GlobalPermissionLevel[]> {
	if (globalPermissionLevels.length === 0) {
		try {
			globalPermissionLevels = await invoke<GlobalPermissionLevel[]>(
				'plugin:author|get_global_permission_levels'
			);
		} catch (e) {
			console.log(e);
			return Promise.reject(e);
		}
	}

	return Promise.resolve(globalPermissionLevels);
}
