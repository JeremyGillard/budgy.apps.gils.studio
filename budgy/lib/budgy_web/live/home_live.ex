defmodule BudgyWeb.HomeLive do
  use BudgyWeb, :live_view

  alias Budgy.Transaction

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(stats: nil)
      |> assign(content: [])
      |> allow_upload(:csv_files, accept: ~w(.csv), max_entries: 10)

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
      |> Enum.filter(fn
        %Transaction{posting_date: %Date{year: 2024}} -> true
        _ -> false
      end)
      |> Enum.sort_by(& &1.number)

    transactions =
      content
      |> Enum.filter(fn transaction -> transaction.number != nil end)

    n_min =
      transactions
      |> Enum.min_by(& &1.number)
      |> Map.get(:number)

    n_max =
      transactions
      |> Enum.max_by(& &1.number)
      |> Map.get(:number)

    transactions_without_n =
      content
      |> Enum.filter(fn transaction -> transaction.number == nil end)

    missings = Transaction.find_missings(transactions)

    stats = %{
      number_of_transation: length(content),
      transaction_missings: Transaction.format_human_readable(missings),
      n_missings: length(transactions_without_n),
      n_min: n_min,
      n_max: n_max
    }

    socket =
      socket
      |> assign(stats: stats)
      |> assign(content: content)

    {:noreply, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="p-4" phx-drop-target={@uploads.csv_files.ref}>
      <form id="upload-form" phx-submit="save" phx-change="validate" class="mb-4">
        <.live_file_input class="file-input" upload={@uploads.csv_files} />
        <button type="submit" class="btn">Upload</button>
      </form>

      <div :if={@stats != nil} class="mb-4">
        <div class="stats shadow">
          <div class="stat">
            <div class="stat-title">Number of Transaction</div>
            <div class="stat-value">{@stats.number_of_transation}</div>
            <div class="stat-desc">Incompletions: {@stats.transaction_missings}</div>
          </div>
          <div class="stat">
            <div class="stat-title">Including Missings</div>
            <div class="stat-value">{@stats.n_missings}</div>
            <div class="stat-desc">min: {@stats.n_min}, max: {@stats.n_max}</div>
          </div>
        </div>
      </div>

      <div :if={length(@content) != 0} class="overflow-x-auto h-172 rounded-box border border-base-content/5 bg-base-100">
        <table class="table table-zebra table-xs table-pin-rows table-pin-cols">
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
