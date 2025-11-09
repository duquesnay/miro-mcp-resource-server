use serde::{Deserialize, Serialize};

/// Represents a Miro board
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

/// API response for list boards endpoint
#[derive(Debug, Deserialize)]
pub struct BoardsResponse {
    pub data: Vec<Board>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Request body for creating a board
#[derive(Debug, Serialize)]
pub struct CreateBoardRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response body for single board creation
#[derive(Debug, Deserialize)]
pub struct CreateBoardResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_serialization() {
        let board = Board {
            id: "board-123".to_string(),
            name: "Test Board".to_string(),
            description: Some("A test board".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&board).unwrap();
        assert!(json.contains("board-123"));
        assert!(json.contains("Test Board"));
    }

    #[test]
    fn test_board_deserialization() {
        let json = r#"{
            "id": "board-456",
            "name": "Another Board",
            "description": "Test description",
            "created_at": "2025-01-02T00:00:00Z"
        }"#;

        let board: Board = serde_json::from_str(json).unwrap();
        assert_eq!(board.id, "board-456");
        assert_eq!(board.name, "Another Board");
        assert_eq!(board.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_create_board_request() {
        let request = CreateBoardRequest {
            name: "New Board".to_string(),
            description: Some("New board description".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("New Board"));
        assert!(json.contains("New board description"));
    }

    #[test]
    fn test_create_board_request_without_description() {
        let request = CreateBoardRequest {
            name: "Board Without Desc".to_string(),
            description: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("description")); // Should be skipped when None
        assert!(json.contains("Board Without Desc"));
    }
}
