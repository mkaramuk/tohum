import {createSlice, PayloadAction} from "@reduxjs/toolkit";

type InitialStateType = {
    count : number
}
const initialState: InitialStateType = {
    count: 0
}

const counterSlice = createSlice({
    name : 'counter',
    initialState,
    reducers : {
        incrementCounter : (state, action : PayloadAction<number>) => {
            state.count += action.payload
        },
        decrementCounter : (state, action : PayloadAction<number>) => {
            state.count -= action.payload
        }
    }
})

export default counterSlice
export const { incrementCounter, decrementCounter } = counterSlice.actions;