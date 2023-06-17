import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

function App() {
  const host = window.location.href;
  const isDev = import.meta.env.DEV;

  const devUrl = "http://190.160.15.44:3000/";
  const baseURl = isDev ? devUrl : host;

  const queryClient = useQueryClient();
  const { isLoading, error, data } = useQuery({
    queryKey: ["ping"],
    queryFn: () => fetch(`${baseURl}api/ping`).then((res) => res.text()),
  });

  const {
    isLoading: isCountLoading,
    error: countError,
    data: countData,
  } = useQuery({
    queryKey: ["count"],
    queryFn: () => fetch(`${baseURl}api/count`).then((res) => res.json()),
  });

  const mutation = useMutation({
    mutationFn: () =>
      fetch(`${baseURl}api/count`, {
        method: "POST",
      }).then((res) => res.json()),
    onSuccess(data) {
      queryClient.setQueryData(["count"], data);
    },
  });

  if (isLoading) return "Loading...";

  if (error || countError)
    return "An error has occurred: " + error + countError;

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        {!countError ? (
          <button
            disabled={mutation.isLoading}
            onClick={() => mutation.mutate()}
          >
            {isCountLoading || mutation.isLoading
              ? "loading..."
              : `count is ${countData.count}`}
          </button>
        ) : null}
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>{" "}
        <p>{data} </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
