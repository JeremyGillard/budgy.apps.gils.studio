defmodule BudgyWeb.CounterpartyLive.Index do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Listing Counterparties
        <:actions>
          <.button variant="primary" navigate={~p"/counterparties/new"}>
            <.icon name="hero-plus" /> New Counterparty
          </.button>
        </:actions>
      </.header>

      <.table
        id="counterparties"
        rows={@streams.counterparties}
        row_click={fn {_id, counterparty} -> JS.navigate(~p"/counterparties/#{counterparty}") end}
      >
        <:col :let={{_id, counterparty}} label="Name">{counterparty.name}</:col>
        <:col :let={{_id, counterparty}} label="Account">{counterparty.account}</:col>
        <:action :let={{_id, counterparty}}>
          <div class="sr-only">
            <.link navigate={~p"/counterparties/#{counterparty}"}>Show</.link>
          </div>
          <.link navigate={~p"/counterparties/#{counterparty}/edit"}>Edit</.link>
        </:action>
        <:action :let={{id, counterparty}}>
          <.link
            phx-click={JS.push("delete", value: %{id: counterparty.id}) |> hide("##{id}")}
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
     |> assign(:page_title, "Listing Counterparties")
     |> stream(:counterparties, Banking.list_counterparties())}
  end

  @impl true
  def handle_event("delete", %{"id" => id}, socket) do
    counterparty = Banking.get_counterparty!(id)
    {:ok, _} = Banking.delete_counterparty(counterparty)

    {:noreply, stream_delete(socket, :counterparties, counterparty)}
  end
end
