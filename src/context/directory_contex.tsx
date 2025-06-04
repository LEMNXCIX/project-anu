import React, {
  createContext,
  useReducer,
  useContext,
  ReactNode,
  Dispatch,
} from "react";
import { DirectoryState, DirEntry } from "@/types/directory_types";

type Action =
  | { type: "SET_ITEMS"; payload: DirEntry[] }
  | { type: "SET_INITIAL_STATE" }
  | { type: "SET_CURRENT_DIRECTORY"; payload: DirEntry }
  | { type: "SET_HISTORIAL_PATH"; payload: DirEntry[] }
  | { type: "ADD_HISTORIAL_PATH"; payload: DirEntry };

const initialState: DirectoryState = {
  items: [],
  currentDirectory: {
    is_directory: false,
    name: "",
    path: "",
  },
  historialPath: [],
};

const DirectoryContext = createContext<{
  state: DirectoryState;
  dispatch: React.Dispatch<Action>;
}>({
  state: initialState,
  dispatch: () => null,
});

const directoryReducer = (
  state: DirectoryState,
  action: Action
): DirectoryState => {
  switch (action.type) {
    case "SET_ITEMS":
      return { ...state, items: action.payload };
    case "SET_INITIAL_STATE":
      return initialState;
    case "SET_CURRENT_DIRECTORY":
      return { ...state, currentDirectory: action.payload };
    case "SET_HISTORIAL_PATH":
      return { ...state, historialPath: action.payload };
    case "ADD_HISTORIAL_PATH":
      let newHistorial = [...state.historialPath];
      if (action.payload) {
        newHistorial.push(action.payload);
      }
      return { ...state, historialPath: newHistorial };
    default:
      return state;
  }
};

export const DirectoryProvider = ({ children }: { children: ReactNode }) => {
  const [state, dispatch] = useReducer(directoryReducer, initialState);

  return (
    <DirectoryContext.Provider value={{ state: state, dispatch: dispatch }}>
      {children}
    </DirectoryContext.Provider>
  );
};

export const useDirectory = () => useContext(DirectoryContext);
