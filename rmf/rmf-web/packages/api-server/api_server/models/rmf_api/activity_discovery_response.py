# generated by datamodel-codegen:
#   filename:  activity_discovery_response.json

from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field


class Activity(BaseModel):
    category: str = Field(
        ...,
        description="The category of this activity. There must not be any duplicate activity categories per fleet.",
    )
    detail: str = Field(..., description="Details about the behavior of the activity.")
    description_schema: Optional[Dict[str, Any]] = Field(
        default=None, description="The schema for this activity description"
    )


class Datum(BaseModel):
    fleet_name: str = Field(
        ..., description="Name of the fleet that supports these activities"
    )
    activities: List[Activity] = Field(
        ..., description="List of activities that the fleet supports"
    )


class ActivityDiscovery(BaseModel):
    data: Optional[List[Datum]] = None
