import React, { createContext, useReducer, useContext, ReactNode, Dispatch } from "react";
import { AppState } from "@/types/app_types";

type Action = { type: "SET_ITEMS"; payload: AppState };

const initialState: AppState = {
  config_user: [],
  loaded: false,
};

const AppContex = createContext<{
  state: AppState;
  dispatch: React.Dispatch<Action>;
}>({
  state: initialState,
  dispatch: () => null,
});

const appReducer = (state: AppState, action: Action): AppState => {
  switch (action.type) {
    case "SET_ITEMS":
      return { ...state, config_user: action.payload, loaded: !action.payload?.config_user};
    default:
      return state;
  }
};

export const AppProvider = ({ children }: { children: ReactNode }) => {
    const [state, dispatch] = useReducer(appReducer, initialState);
  
    return (
      <AppContex.Provider value={{ state: state, dispatch: dispatch }}>
        {children}
      </AppContex.Provider>
    );
  };

export const useApp = () => useContext(AppContex);
