import React, { createContext, useReducer, useContext, ReactNode, Dispatch } from "react";
import { DirectoryState, DirEntry } from "@/types/directory";

type Action = { type: "SET_ITEMS"; payload: DirEntry[] };

const initialState: DirectoryState = {
  items: [],
};

const DirectoryContext = createContext<{
  state: DirectoryState;
  dispatch: React.Dispatch<Action>;
}>({
  state: initialState,
  dispatch: () => null,
});

const directoryReducer = (state: DirectoryState, action: Action): DirectoryState => {
  switch (action.type) {
    case "SET_ITEMS":
      return { ...state, items: action.payload };
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
