defmodule BudgyWeb.HomeLive do
  use BudgyWeb, :live_view

  # alias Budgy.Transaction
  alias Budgy.Banking

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(records: [])
      |> assign(missing_transaction_numbers: [])
      |> allow_upload(:csv_files,
        accept: ~w(.csv),
        progress: &handle_progress/3,
        auto_upload: true,
        max_entries: 10
      )

    {:ok, socket}
  end

  def handle_progress(:csv_files, entry, socket) do
    if entry.done? do
      uploaded_records =
        consume_uploaded_entry(socket, entry, fn %{path: path} ->
          IO.inspect(entry, label: "Entry")

          record =
            path
            |> File.stream!()
            |> Stream.drop(12)
            |> Stream.map(&:unicode.characters_to_binary(&1, :latin1, :utf8))
            |> CSV.decode!(separator: ?;, headers: true)
            |> Enum.to_list()
            |> Enum.map(fn row -> Banking.Record.from_row(row) end)
            |> Enum.map(fn attrs -> Banking.Record.struct!(attrs) end)

          {:ok, record}
        end)

      records =
        socket.assigns.records
        |> Enum.reverse(uploaded_records)

      socket =
        socket
        |> assign(records: records)

      {:noreply, socket}
    else
      {:noreply, socket}
    end
  end

  @impl true
  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("save", _params, socket) do
    {:noreply, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="p-4" phx-drop-target={@uploads.csv_files.ref}>
      <%!-- <h1>Budgy v{Application.spec(:budgy, :vsn)}</h1> --%>

      <form id="upload-form" phx-submit="save" phx-change="validate" class="mb-4">
        <.live_file_input class="file-input" upload={@uploads.csv_files} />
      </form>

      <%!-- <div class="flex flex-col gap-2">
        <div :for={entry <- @uploads.csv_files.entries}>
          {entry.client_name}
          <progress class="progress w-16" value={entry.progress} max="100"></progress>
        </div>
      </div> --%>

      <div :if={length(@records) != 0}>
        <table class="table table-zebra table-xs table-pin-rows table-pin-cols">
          <thead>
            <tr>
              <th>N</th>
              <th>S</th>
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
            <tr :for={transaction <- @records}>
              <th>{transaction.number}</th>
              <th>{transaction.statement_number}</th>
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

  # defp calculate_stats(records) do
  #   records_without_n =
  #     records
  #     |> Enum.filter(fn transaction -> transaction.number == nil end)

  #   records =
  #     records
  #     |> Enum.filter(fn transaction -> transaction.number != nil end)

  #   n_min =
  #     records
  #     |> Enum.min_by(& &1.number)
  #     |> Map.get(:number)

  #   n_max =
  #     records
  #     |> Enum.max_by(& &1.number)
  #     |> Map.get(:number)

  #   missings = Transaction.find_missings(records)

  #   %{
  #     number_of_transation: length(records),
  #     transaction_missings: Transaction.format_human_readable(missings),
  #     n_missings: length(records_without_n),
  #     n_min: n_min,
  #     n_max: n_max
  #   }
  # end
end
