# generated by datamodel-codegen:
#   filename:  resume_task_request.json

from __future__ import annotations

from typing import List, Optional

from pydantic import BaseModel, Field
from typing_extensions import Literal


class TaskResumeRequest(BaseModel):
    type: Optional[Literal["resume_task_request"]] = Field(
        default=None, description="Indicate that this is a task resuming request"
    )
    for_task: Optional[str] = Field(
        default=None, description="Specify task ID to resume."
    )
    for_tokens: Optional[List[str]] = Field(
        default=None,
        description="A list of tokens of interruption requests which should be resumed. The interruption request associated with each token will be discarded.",
        min_length=1,
    )
    labels: Optional[List[str]] = Field(
        default=None, description="Labels describing this request"
    )