defmodule BudgyWeb.BankLive.Index do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Listing Banks
        <:actions>
          <.button variant="primary" navigate={~p"/banks/new"}>
            <.icon name="hero-plus" /> New Bank
          </.button>
        </:actions>
      </.header>

      <.table
        id="banks"
        rows={@streams.banks}
        row_click={fn {_id, bank} -> JS.navigate(~p"/banks/#{bank}") end}
      >
        <:col :let={{_id, bank}} label="Name">{bank.name}</:col>
        <:col :let={{_id, bank}} label="Bic">{bank.bic}</:col>
        <:col :let={{_id, bank}} label="Country code">{bank.country_code}</:col>
        <:action :let={{_id, bank}}>
          <div class="sr-only">
            <.link navigate={~p"/banks/#{bank}"}>Show</.link>
          </div>
          <.link navigate={~p"/banks/#{bank}/edit"}>Edit</.link>
        </:action>
        <:action :let={{id, bank}}>
          <.link
            phx-click={JS.push("delete", value: %{id: bank.id}) |> hide("##{id}")}
            data-confirm="Are you sure?"
          >
            Delete
          </.link>
        </:action>
      </.table>
    </Layouts.app>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Listing Banks")
     |> stream(:banks, Banking.list_banks())}
  end

  @impl true
  def handle_event("delete", %{"id" => id}, socket) do
    bank = Banking.get_bank!(id)
    {:ok, _} = Banking.delete_bank(bank)

    {:noreply, stream_delete(socket, :banks, bank)}
  end
end
