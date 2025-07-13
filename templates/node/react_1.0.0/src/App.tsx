import {useDispatch, useSelector} from "react-redux";
import {RootState} from "./redux/store.ts";
import {decrementCounter, incrementCounter} from "./redux/slices/counter.ts";

function App() {
    const count = useSelector((state : RootState) => state.counter.count);
    const dispatch = useDispatch();

    const increment = () => {
        dispatch(incrementCounter(1));
    }

    const decrement = () => {
        dispatch(decrementCounter(1));
    }

    return (
        <>
            <main className={`bg-[#222222] w-full h-screen flex flex-col gap-y-4 justify-center items-center`}>
                <div className={`flex items-center justify-center gap-x-4`}>
                    <span className={`text-white text-xl font-bold`}>Counter :</span>
                    <button
                        onClick={increment}
                        className={`text-white border border-zinc-200 w-12 h-12 bg-[#111111] text-2xl font-bold rounded-md cursor-pointer active:scale-95`}>+
                    </button>
                    <span className={`text-2xl font-bold text-white`}>{count}</span>
                    <button
                        onClick={decrement}
                        className={`text-white border border-zinc-200 w-12 h-12 bg-[#111111] text-2xl font-bold rounded-md cursor-pointer active:scale-95`}>-
                    </button>
                </div>
                <span className={`border border-zinc-200 px-2 flex flex-col items-center gap-y-2 text-white font-thin text-lg`}>
                    <kbd>redux packages installed (react-redux , redux , @reduxjs/toolkit)</kbd>
                    <span>redux files in src/redux</span>
                    <kbd>tailwindcss@^4.1.3 installed</kbd>
                </span>
            </main>
        </>
    )
}

export default App
