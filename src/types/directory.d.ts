export interface DirEntry {
  name: string;
  path?: string;
  is_directory: boolean;
}

export interface ListDirectory {
  entries: DirEntry[];
  path: string;
}

export interface DirectoryState {
    items: DirEntry[];
  }
  