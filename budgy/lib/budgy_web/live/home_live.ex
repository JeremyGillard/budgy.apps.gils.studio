defmodule BudgyWeb.HomeLive do
  use BudgyWeb, :live_view

  alias Budgy.Transaction

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(content: [])
      |> allow_upload(:csv_files, accept: ~w(.csv), max_entries: 5)

    {:ok, socket}
  end

  @impl true
  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("save", _params, socket) do
    uploaded_files =
      consume_uploaded_entries(socket, :csv_files, fn %{path: path}, _entry ->
        content =
          path
          |> File.stream!()
          |> Stream.drop(12)
          |> Stream.map(&:unicode.characters_to_binary(&1, :latin1, :utf8))
          |> CSV.decode!(separator: ?;, headers: true)
          |> Enum.to_list()
          |> Enum.map(fn original -> Transaction.from_original(original) end)
          |> Enum.map(fn attrs -> Transaction.to_struct(attrs) end)

        {:ok, content}
      end)

    content =
      uploaded_files
      |> List.flatten()
      |> Enum.sort_by(& &1.number)

    IO.inspect(content, label: "Content")

    {:noreply, assign(socket, content: content)}
  end

  def handle_event("show", _params, socket) do
    IO.inspect(length(socket.assigns.content))
    IO.inspect(socket.assigns.content |> List.first())
    {:noreply, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div phx-drop-target={@uploads.csv_files.ref}>
      <form id="upload-form" phx-submit="save" phx-change="validate">
        <.live_file_input class="file-input" upload={@uploads.csv_files} />
        <button type="submit" class="btn">Upload</button>
      </form>
      <button class="btn" phx-click="show">Show</button>
      <div :if={length(@content) != 0} class="overflow-x-auto">
        <table class="table table-xs">
          <thead>
            <tr>
              <th>N</th>
              <th>Name</th>
              <th>Account</th>
              <th>Name (counterpart)</th>
              <th>Account (counterpart)</th>
              <th>Street</th>
              <th>Postal Code</th>
              <th>BIC</th>
              <th>Country Code</th>
              <th>Amount</th>
              <th>Currency</th>
              <th>Date of recognition</th>
              <th>Value Date</th>
              <th>Description</th>
              <th>Communication</th>
            </tr>
          </thead>
          <tbody>
            <tr :for={transaction <- @content}>
              <th>{transaction.number}</th>
              <td>{transaction.name}</td>
              <td>{transaction.account}</td>
              <td>{transaction.counterpart_name}</td>
              <td>{transaction.counterpart_account}</td>
              <td>{transaction.counterpart_street}</td>
              <td>{transaction.counterpart_postal_code}</td>
              <td>{transaction.counterpart_bic}</td>
              <td>{transaction.counterpart_country_code}</td>
              <td>{transaction.amount}</td>
              <td>{transaction.currency}</td>
              <td>{transaction.posting_date}</td>
              <td>{transaction.value_date}</td>
              <td>{transaction.description}</td>
              <td>{transaction.communication}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    """
  end
end
